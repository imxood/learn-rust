use eframe::{egui, epi};
use egui::{
    epaint::{Mesh, Vertex},
    vec2, Color32, Pos2, Rect, Sense, Shape,
};
use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
        tokio::spawn(async {
            println!("async task begin...");
            tokio::time::sleep(Duration::from_millis(2000)).await;
            println!("async task end");
        })
        .await
        .unwrap();

        let blocking_task = tokio::task::spawn_blocking(|| {
            println!("blocking task begin...");
            std::thread::sleep(Duration::from_millis(5000));
            println!("blocking task end");
        });

        blocking_task.await.unwrap();
    });
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}

#[derive(Default)]
struct MyApp {}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "My egui App"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("test paint UV");

            // 画一个 铺满有效区域 的矩形
            let (_res, painter) =
                ui.allocate_painter(ui.available_rect_before_wrap().size(), Sense::click());
            let rect = painter.clip_rect();
            let mut mesh = Mesh::default();
            mesh.add_colored_rect(rect, Color32::LIGHT_BLUE);

            // 画一个 小矩形

            let left_top = rect.left_top();
            let right_bottom = left_top + vec2(100.0, 100.0);
            let rect = Rect::from_min_max(left_top, right_bottom);
            mesh.add_colored_rect(rect, Color32::LIGHT_YELLOW);

            // 画另一个 矩形

            // 连续的顶点索引构成一个完整的形状， 每3个三角形构成一个形状, 矩形需要两个三角形
            let idx = mesh.vertices.len() as u32;
            let rect = rect.translate(vec2(100.0, 100.0));
            println!("rect: {:?}", &rect);
            mesh.indices
                .extend([idx, idx + 1, idx + 2, idx, idx + 3, idx + 2]);
            mesh.vertices.extend([
                Vertex {
                    pos: rect.left_top(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
                Vertex {
                    pos: rect.right_top(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
                Vertex {
                    pos: rect.right_bottom(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
                Vertex {
                    pos: rect.left_bottom(),
                    uv: Pos2::ZERO,
                    color: Color32::LIGHT_RED,
                },
            ]);

            // 画一个 线段

            let rect = rect.translate(vec2(100.0, 100.0));
            let left_top = rect.left_top();
            let right_bottom = left_top + vec2(100.0, 1.0);
            let rect = Rect::from_min_max(left_top, right_bottom);
            mesh.add_colored_rect(rect, Color32::LIGHT_YELLOW);

            painter.add(Shape::Mesh(mesh));
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}
