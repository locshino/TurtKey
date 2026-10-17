//
// TurtKey - Cross-platform Vietnamese Input Engine and GUI in Rust
// Based on OpenKey (Mai Vu Tuyen)
// Copyright (c) 2026 TurtKey Contributors
//

pub mod app;
pub mod engine;
pub mod i18n;
pub mod plugins;

use app::TurtKeyApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0, 580.0])
            .with_min_inner_size([500.0, 450.0])
            .with_title("TurtKey - Bộ gõ Tiếng Việt"),
        ..Default::default()
    };

    eframe::run_native(
        "TurtKey",
        native_options,
        Box::new(|cc| Ok(Box::new(TurtKeyApp::new(cc)))),
    )
}
