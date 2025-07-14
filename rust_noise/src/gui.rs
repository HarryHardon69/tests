use crate::noise::{Noise, NoiseType};
use eframe::{egui, epi};
use image::{ImageBuffer, Rgba};
use noise::{
    self,
    NoiseFn,
    Point2,
};

pub struct NoiseGui {
    noise: Noise,
    texture: Option<egui::TextureHandle>,
    seed: u32,
}

impl Default for NoiseGui {
    fn default() -> Self {
        Self {
            noise: Noise::default(),
            texture: None,
            seed: 0,
        }
    }
}

impl epi::App for NoiseGui {
    fn name(&self) -> &str {
        "Rust Noise"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.generate_texture(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let needs_new_texture = egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("Noise Parameters");

                let mut changed = false;

                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.octaves, 1..=16).text("Octaves"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.frequency, 0.0..=1.0).text("Frequency"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.lacunarity, 1.0..=4.0).text("Lacunarity"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.persistence, 0.0..=2.0).text("Persistence"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.gain, 0.0..=1.0).text("Gain"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut self.seed, 0..=1000).text("Seed"))
                    .changed();

                egui::ComboBox::from_label("Noise Type")
                    .selected_text(format!("{:?}", self.noise.noise_type))
                    .show_ui(ui, |ui| {
                        changed |= ui.selectable_value(
                            &mut self.noise.noise_type,
                            NoiseType::Value,
                            "Value",
                        ).changed();
                        changed |= ui.selectable_value(
                            &mut self.noise.noise_type,
                            NoiseType::Perlin,
                            "Perlin",
                        ).changed();
                        changed |= ui.selectable_value(
                            &mut self.noise.noise_type,
                            NoiseType::Simplex,
                            "Simplex",
                        ).changed();
                    });

                if ui.button("Export").clicked() {
                    self.export_wallpaper();
                }

                if let Some(texture) = &self.texture {
                    ui.image(texture, texture.size_vec2());
                }

                changed
            })
            .inner;

        if needs_new_texture {
            self.generate_texture(ctx);
        }
    }
}

impl NoiseGui {
    fn generate_texture(&mut self, ctx: &egui::CtxRef) {
        let size = 256;
        let mut image = ImageBuffer::new(size, size);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let value = self.get_noise(x as f64, y as f64);
            let color = ((value + 1.0) / 2.0 * 255.0) as u8;
            *pixel = Rgba([color, color, color, 255]);
        }

        let image_data = image.into_raw();
        let texture = ctx.alloc_texture(egui::ColorImage::from_rgba_unmultiplied(
            [size as usize, size as usize],
            &image_data,
        ));
        self.texture = Some(texture);
    }

    fn get_noise(&self, x: f64, y: f64) -> f64 {
        let point = [x, y];
        let mut value = 0.0;
        let mut frequency = self.noise.frequency;
        let mut amplitude = self.noise.gain;

        for _ in 0..self.noise.octaves {
            let sample = match self.noise.noise_type {
                NoiseType::Value => noise::Value::new(self.seed).get([point[0] * frequency, point[1] * frequency]),
                NoiseType::Perlin => noise::Perlin::new(self.seed).get([point[0] * frequency, point[1] * frequency]),
                NoiseType::Simplex => noise::Simplex::new(self.seed).get([point[0] * frequency, point[1] * frequency]),
            };
            value += sample * amplitude;
            frequency *= self.noise.lacunarity;
            amplitude *= self.noise.persistence;
        }
        value
    }

    fn export_wallpaper(&self) {
        let size = 1920;
        let mut image = ImageBuffer::new(size, size);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let value = self.get_noise(x as f64, y as f64);
            let color = ((value + 1.0) / 2.0 * 255.0) as u8;
            *pixel = Rgba([color, color, color, 255]);
        }

        image.save("wallpaper.png").unwrap();
    }
}
