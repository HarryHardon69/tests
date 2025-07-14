mod noise;
mod gui;

use gui::NoiseGui;
use eframe::epi::App;

fn main() {
    let app = NoiseGui::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
