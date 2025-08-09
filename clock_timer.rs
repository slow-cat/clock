use chrono::{self, Timelike};
use eframe::{App, egui};
use egui::{Color32, Pos2, Stroke, Vec2};
use std::{
    f32::consts::TAU,
    time::{Duration, Instant},
};
fn polar_to_cartesian(center: Pos2, length: f32, angle: f32) -> Pos2 {
    let x = center.x + angle.sin() * length;
    let y = center.y - angle.cos() * length;
    Pos2::new(x, y)
}
fn draw_frame(center: Pos2, radius: f32, painter: egui::Painter) {
    painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::GRAY));
    for i in 0..12 {
        let angle = i as f32 * TAU / 12.0;
        let start = polar_to_cartesian(center, radius - 10.0, angle);
        let end = polar_to_cartesian(center, radius, angle);
        painter.line_segment([start, end], Stroke::new(2.0, Color32::GRAY));
    }
}
fn draw_line(
    center: Pos2,
    radius: f32,
    radian: f32,
    painter: egui::Painter,
    thickness: f32,
    color: Color32,
) {
    let end = polar_to_cartesian(center, radius, radian);
    painter.line_segment([center, end], Stroke::new(thickness, color));
}
fn clock(rect: egui::Rect, second: f32, minute: f32, hour: f32, ui: &mut egui::Ui) {
    let painter = ui.painter_at(rect);

    let center = rect.center();
    let radius = rect.width().min(rect.height()) / 2.0 - 10.0;
    draw_frame(center, radius, painter.clone());
    draw_line(
        center,
        radius * 0.9,
        (second / 60.0) * TAU,
        painter.clone(),
        1.5,
        Color32::RED,
    );
    draw_line(
        center,
        radius * 0.7,
        (minute / 60.0) * TAU,
        painter.clone(),
        3.0,
        Color32::LIGHT_BLUE,
    );
    draw_line(
        center,
        radius * 0.5,
        (hour / 12.0) * TAU,
        painter.clone(),
        4.0,
        Color32::WHITE,
    );
}
struct StopwatchApp {
    start_time: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Default for StopwatchApp {
    fn default() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::ZERO,
            running: false,
        }
    }
}
impl App for StopwatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let local_time = chrono::Local::now();
            let second = local_time.second() as f32;
            let minute = local_time.minute() as f32 + second / 60.0;
            let hour = local_time.hour12().1 as f32 + minute / 60.0;
            let display_time = if self.running {
                self.elapsed + self.start_time.unwrap().elapsed()
            } else {
                self.elapsed
            };
            let disp_second = display_time.as_secs_f32() % 60.0;
            let disp_minute = (display_time.as_secs_f32() / 60.0) % 60.0;
            let disp_hour = (display_time.as_secs_f32() / 3060.0) % 12.0;
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(300.0), egui::Sense::hover());
            clock(rect, second, minute, hour, ui);

            ui.heading(format!("{}", local_time.format("%H:%M:%S")));
            ui.separator();

            let (rect2, _) = ui.allocate_exact_size(Vec2::splat(300.0), egui::Sense::hover());
            clock(rect2, disp_second, disp_minute, disp_hour, ui);
            ui.label(format!(
                "{:02}:{:02}.{:03}",
                (display_time.as_secs() / 60),
                (display_time.as_secs() % 60),
                (display_time.subsec_millis())
            ));
            ui.horizontal(|ui| {
                if ui
                    .button(if self.running { "halt" } else { "start" })
                    .clicked()
                {
                    if self.running {
                        self.elapsed += self.start_time.unwrap().elapsed();
                        self.running = false;
                    } else {
                        self.start_time = Some(Instant::now());
                        self.running = true;
                    }
                }
                if ui.button("rset").clicked() {
                    self.running = false;
                    self.elapsed = Duration::ZERO;
                    self.start_time = None;
                }
            });
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        });
    }
}
fn main() -> eframe::Result<()> {
    let native_option = eframe::NativeOptions::default();
    eframe::run_native(
        "clock",
        native_option,
        Box::new(|_cc| Ok(Box::new(StopwatchApp::default()))),
    )
}
