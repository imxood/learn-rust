use anyhow::{bail, Result};
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use realsense_rust::{
  config::Config,
  context::Context,
  frame::{ColorFrame, DepthFrame},
  kind::{Rs2CameraInfo, Rs2Format, Rs2Option, Rs2ProductLine, Rs2StreamKind},
  pipeline::{ActivePipeline, InactivePipeline},
  sensor::Sensor,
};
use serde_json::Value;
use std::{
  collections::HashSet, convert::TryFrom, fmt::Debug, fs::File, io::Read, task::Poll,
  time::Duration, u8,
};

pub fn d400_color_data(color_frame: &ColorFrame) -> Vec<u8> {
  let data_size = color_frame.get_data_size();
  let mut color_data = vec![0; data_size];
  unsafe {
    let data = color_frame.get_data() as *const std::os::raw::c_void as *const u8;
    let data = std::slice::from_raw_parts(data, data_size);
    color_data.copy_from_slice(data);
  }
  color_data
}

pub fn d400_depth_data(depth_frame: &DepthFrame) -> Vec<u16> {
  let data_size = depth_frame.get_data_size() / 2;
  let mut depth_data = vec![0u16; data_size];
  unsafe {
    let data = depth_frame.get_data() as *const std::ffi::c_void as *const u16;
    let data = std::slice::from_raw_parts(data, data_size);
    depth_data.copy_from_slice(data);
  }
  depth_data
}

fn camera_load_config(sensor: &mut Sensor, configfile: &str) -> Value {
  let mut file_content = String::new();
  File::open(configfile)
    .unwrap()
    .read_to_string(&mut file_content)
    .unwrap();

  let obj: Value = serde_json::from_str(&file_content).unwrap();
  // log::info!("json obj: {:?}", &obj);

  /*
      设置 深度相机 参数
  */

  //   let v = obj["controls-autoexposure-auto"].as_bool().unwrap();

  //   // 设置 自动曝光
  //   sensor.set_option(Rs2Option::EnableAutoExposure, if v { 1.0 } else { 0.0 })?;

  /*
      设置 RGB相机 参数
  */

  let v = obj["controls-color-autoexposure-auto"].as_bool().unwrap();

  // 设置 自动曝光
  sensor
    .set_option(Rs2Option::EnableAutoExposure, if v { 1.0 } else { 0.0 })
    .unwrap();

  // 设置 Brightness
  sensor
    .set_option(
      Rs2Option::Brightness,
      obj["controls-color-brightness"]
        .as_str()
        .unwrap()
        .parse::<f32>()
        .unwrap(),
    )
    .unwrap();

  // 设置 Contrast
  sensor
    .set_option(
      Rs2Option::Contrast,
      obj["controls-color-contrast"]
        .as_str()
        .unwrap()
        .parse::<f32>()
        .unwrap(),
    )
    .unwrap();

  // 设置 EXPOSURE
  sensor
    .set_option(
      Rs2Option::Exposure,
      obj["controls-color-autoexposure-manual"]
        .as_str()
        .unwrap()
        .parse::<f32>()
        .unwrap(),
    )
    .unwrap();

  // 设置 Gain
  sensor
    .set_option(
      Rs2Option::Gain,
      obj["controls-color-gain"]
        .as_str()
        .unwrap()
        .parse::<f32>()
        .unwrap(),
    )
    .unwrap();

  log::info!("camera config successfully");
  obj
}

fn camera_open(configfile: &str) -> Result<ActivePipeline> {
  let context = Context::new().unwrap();

  let mut queryable_set = HashSet::new();
  queryable_set.insert(Rs2ProductLine::D400);

  // 查询 D400系列 的相机
  let mut devices = context.query_devices(queryable_set);

  if devices.is_empty() {
    bail!("can't find devices");
  }

  // 只使用第一个相机
  let device = devices.swap_remove(0);

  // 查询 序列号
  let serial = device.info(Rs2CameraInfo::SerialNumber).unwrap();
  let serial_number = serial.to_string_lossy().into_owned();

  // 查询 usb类型
  let usb_cstr = device.info(Rs2CameraInfo::UsbTypeDescriptor).unwrap();
  let usb_type: f32 = usb_cstr.to_str().unwrap().parse().unwrap();

  log::info!("usb type: {:?}", usb_type);
  log::info!("serial_number: {:?}", &serial_number);

  // 查看 模块数量
  let mut sensors = device.sensors();
  log::info!("sensors len: {}", sensors.len());

  // First sensor is the stereo module
  // Second sensor is the rgb module

  // RGB相机
  let mut sensor = sensors.get_mut(1).unwrap();

  let obj = camera_load_config(&mut sensor, configfile);

  let width = obj["stream-width"]
    .as_str()
    .unwrap()
    .parse::<usize>()
    .unwrap();
  let height = obj["stream-height"]
    .as_str()
    .unwrap()
    .parse::<usize>()
    .unwrap();
  let framerate = obj["stream-fps"]
    .as_str()
    .unwrap()
    .parse::<usize>()
    .unwrap();
  // let depth_format = obj["stream-depth-format"]
  //     .as_str()
  //     .unwrap()
  //     .parse::<Rs2Format>()
  //     .unwrap();

  /*
      打开摄像头
  */

  let mut config = Config::new();
  config
    .enable_device_from_serial(serial)?
    .disable_all_streams()?
    .enable_stream(
      Rs2StreamKind::Depth,
      None,
      width,
      height,
      Rs2Format::Z16,
      framerate,
    )?
    .enable_stream(
      Rs2StreamKind::Color,
      None,
      width,
      height,
      Rs2Format::Rgb8,
      framerate,
    )?;
  let pipeline = InactivePipeline::try_from(&context).unwrap();

  //   if !pipeline.can_resolve(&config) {
  //     bail("pipeline can't be resolve");
  //   }

  //   assert!(pipeline.resolve(&config).is_some());

  let pipeline = pipeline.start(Some(config))?;

  Ok(pipeline)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ColorRGB {
  r: u8,
  g: u8,
  b: u8,
}

pub struct CameraFrame {
  // RGB相机 u8 RGB数据, width * height * 3
  pub color_data: Vec<u8>,
  // Depth相机 u16 Depth数据, width * height * 2
  pub depth_data: Vec<u16>,
  pub depth_units: f32,
  // x方向像素数
  pub width: usize,
  // y方向像素数
  pub height: usize,
}

impl Debug for CameraFrame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CameraFrame")
      .field("width", &self.width)
      .field("height", &self.height)
      .field("depth_units", &self.depth_units)
      .finish()
  }
}

impl CameraFrame {
  fn new(
    color_data: Vec<u8>,
    depth_data: Vec<u16>,
    depth_units: f32,
    width: usize,
    height: usize,
  ) -> Self {
    Self {
      color_data,
      depth_data,
      depth_units,
      width,
      height,
    }
  }

  /// 每一个距离信息, 都会转换成一个RGB颜色
  /// RGB数组 的 总长度 为 depth_data长度的3倍
  /// [R,G,B,R,G,B,...]
  pub fn to_rgb(
    &mut self,
    min_distance: f32,
    max_distance: f32,
    area: (usize, usize, usize, usize),
    fill_color: Option<u32>,
  ) -> Vec<u8> {
    let (area_start_x, area_start_y, mut area_start_w, mut area_start_h) = area;
    if area_start_w == 0 {
      area_start_w = self.width;
    }
    if area_start_h == 0 {
      area_start_h = self.height;
    }
    let depth_units = self.depth_units;
    let color = fill_color.unwrap_or_default();

    let colors = unsafe {
      std::slice::from_raw_parts_mut(
        self.color_data.as_mut_ptr() as *mut ColorRGB,
        self.width * self.height,
      )
    };

    self
      .depth_data
      .par_iter()
      .zip(colors.par_iter_mut())
      .for_each(|(d, c)| {
        // 距离, 单位: m
        let x = (*d as f32) * depth_units;
        // 如果不在深度范围内, 则为黑色
        if x < min_distance || x > max_distance {
          c.r = 0;
          c.g = 0;
          c.b = 0;
        }
        // 在范围内, 如果有填充色, 则填充
        else if fill_color.is_some() {
          c.r = ((color >> 16) & 0xff) as u8;
          c.g = ((color >> 8) & 0xff) as u8;
          c.b = (color & 0xff) as u8;
        }
        // 在范围内, 没有填充色, 就使用rgb帧中的颜色
      });

    let len = area_start_w * area_start_h;
    let mut data = vec![0u8; len * 3];

    {
      let p = data.as_mut_ptr() as *mut ColorRGB;
      let rgb_data = unsafe { std::slice::from_raw_parts_mut(p, len) };

      for i in 0..area_start_h {
        let y1 = area_start_y + i;
        let pos_start = y1 * self.width + area_start_x;
        let colors = &colors[pos_start..pos_start + area_start_w];

        let c = &mut rgb_data[i * area_start_w..(i + 1) * area_start_w];
        c.copy_from_slice(colors);
        c.reverse();
      }
    }
    data

    // let rgb_data_len = rgb_data.len() * 3;
    // let rgb_data_cap = rgb_data.capacity() * 3;
    // let rgb_data_p = rgb_data.leak() as *mut _ as *mut u8;

    // unsafe { Vec::from_raw_parts(rgb_data_p, rgb_data_len, rgb_data_cap) }
  }

  pub fn has_person(&self, rgb_data: &[u8], person_pixel_number: usize) -> bool {
    let mut count = 0;
    let len = rgb_data.len() / 3;
    for i in 0..len {
      let r = rgb_data[i * 3] as u32;
      let g = rgb_data[i * 3 + 1] as u32;
      let b = rgb_data[i * 3 + 2] as u32;
      let rgb = (r << 16) + (g << 8) + b;
      if rgb != 0 {
        count += 1;
      }
    }
    count > person_pixel_number
  }
}

pub struct CameraService {
  pub pipeline: Option<ActivePipeline>,
  pub configfile: String,
}

impl CameraService {
  pub fn new(configfile: String) -> Self {
    Self {
      pipeline: None,
      configfile,
    }
  }

  pub fn set_configfile(&mut self, configfile: &str) {
    self.configfile = configfile.to_string();
  }

  pub fn open(&mut self) -> Result<()> {
    if self.pipeline.is_none() {
      log::info!("使用相机配置文件: {:?}", &self.configfile);
      self.pipeline = Some(camera_open(&self.configfile)?);
    }
    Ok(())
  }

  pub fn close(&mut self) {
    if self.pipeline.is_some() {
      self.pipeline = None;
    }
  }

  pub fn poll_frame(&mut self) -> Result<Option<CameraFrame>> {
    self.open()?;
    let camera_frame = self.pipeline.as_mut().unwrap().poll()?;
    match camera_frame {
      Poll::Ready(frame) => {
        let mut depth_frames = frame.frames_of_type::<DepthFrame>();
        let mut color_frames = frame.frames_of_type::<ColorFrame>();

        let depth_frame = depth_frames.pop().unwrap();
        let color_frame = color_frames.pop().unwrap();

        let depth_data = d400_depth_data(&depth_frame);
        let color_data = d400_color_data(&color_frame);

        Ok(Some(CameraFrame::new(
          color_data,
          depth_data,
          depth_frame.depth_units().unwrap(),
          color_frame.width(),
          color_frame.height(),
        )))
      }
      Poll::Pending => Ok(None),
    }
  }

  pub fn fetch_frame(&mut self, timeout: Option<Duration>) -> Result<CameraFrame> {
    self.open()?;
    let camera_frame = self.pipeline.as_mut().unwrap().wait(timeout)?;
    let mut depth_frames = camera_frame.frames_of_type::<DepthFrame>();
    let mut color_frames = camera_frame.frames_of_type::<ColorFrame>();

    let depth_frame = depth_frames.pop().unwrap();
    let color_frame = color_frames.pop().unwrap();

    let depth_data = d400_depth_data(&depth_frame);
    let color_data = d400_color_data(&color_frame);

    Ok(CameraFrame::new(
      color_data,
      depth_data,
      depth_frame.depth_units().unwrap(),
      color_frame.width(),
      color_frame.height(),
    ))
  }
}
