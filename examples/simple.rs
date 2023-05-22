#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui::*};
use eframe::egui;

use egui_clip_textedit::*;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    print!("eframe rederer default : {}", eframe::Renderer::default());

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        //#[cfg(feature = "wgpu")]
        //renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };


    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    text_editor: ClipTextEdit,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_editor: ClipTextEdit::new(String::from("Test\nHello World")),
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Editor");


            self.text_editor.show_editor(ui, ui.available_rect_before_wrap());

        });
    }
}