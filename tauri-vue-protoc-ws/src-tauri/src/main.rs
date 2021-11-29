#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::time::Instant;

use anyhow::Result;
use flexi_logger::{DeferredNow, Logger, Record};
use protobuf::Message;
use tauri::Manager;

use crate::{
  protos::example::{get_response::Status, GetRequest, GetResponse},
  web::start_web,
};

mod binary;
mod protos;
mod req;
mod web;

fn main() -> Result<()> {
  init_log()?;
  test_proto();

  tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();
      let now = Instant::now();
      main_window.listen("init", move |event| {
        log::info!("delta: {:?}", now.elapsed());
        log::info!("got window event-name with payload {:?}", event.payload());
      });
      // 启动 web 服务
      tauri::async_runtime::spawn(async move {
        start_web("127.0.0.1", 8080).await.unwrap();
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}

fn init_log() -> Result<()> {
  std::env::set_var("RUST_LOG", "DEBUG");
  Logger::try_with_str("debug")?
    .log_to_file(flexi_logger::FileSpec::default().directory("logs")) // 把日志写到文件中
    .duplicate_to_stderr(flexi_logger::Duplicate::All) // 警告和错误发送到控制台
    .format(opt_format)
    .start()?;
  Ok(())
}

pub fn opt_format(
  w: &mut dyn std::io::Write,
  now: &mut DeferredNow,
  record: &Record,
) -> core::result::Result<(), std::io::Error> {
  write!(
    w,
    "[{}] {:5} [{}:{}] {}",
    now.now().format("%m-%d %H:%M:%S%.3f"),
    record.level(),
    record.module_path().unwrap_or("<unnamed>"),
    record.line().unwrap_or(0),
    &record.args()
  )
}

fn test_proto() {
  // 构造一个对象
  let mut out_msg = GetRequest::new();
  log::info!("GetRequest::new: {:?}", serde_json::to_string(&out_msg));

  out_msg.name = "John Smith".into();
  out_msg.age = 25;
  out_msg.features.push("one".into());
  out_msg.features.push("two".into());

  // 从 对象 转 字节数组
  let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();
  log::info!("out_msg: {:?}", &out_msg);
  log::info!("out_bytes: {:?}", &out_bytes);

  // let json_data = serde_json::to_string(&out_msg).unwrap();
  // log::info!("json_data: {:?}", &json_data);

  // 从 字节数组 转 对象
  let in_msg: GetRequest = Message::parse_from_bytes(&out_bytes).unwrap();

  assert_eq!(in_msg.name, "John Smith");

  //////////////////////////////////

  // 构造一个对象
  let mut out_resp = GetResponse::new();
  out_resp.status = Status::OK.into();
  out_resp.address = "1243 main street".into();
  out_resp.city = "anytown".into();
  out_resp.zipcode = 54321;
  out_resp.data = vec![1, 2, 3, 4, 5, 6];

  // 从 对象 转 字节数组
  let out_bytes: Vec<u8> = out_resp.write_to_bytes().unwrap();

  log::info!("out_resp: {:?}", &out_resp);
  log::info!("out_bytes: {:?}", &out_bytes);

  // 从 字节数组 转 对象
  let in_resp: GetResponse = Message::parse_from_bytes(&out_bytes).unwrap();

  // let json_data = serde_json::to_string(&out_resp).unwrap();
  // log::info!("json_data: {:?}", &json_data);

  assert_eq!(in_resp.status, out_resp.status);
  assert_eq!(in_resp.zipcode, out_resp.zipcode);
  assert_eq!(in_resp.address, out_resp.address);

  // 转 json
  log::info!("in_resp json: {:?}", serde_json::to_string(&in_resp));
}
