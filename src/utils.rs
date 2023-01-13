use egui::{Color32, Vec2};
use rand::Rng;

use crate::{
    app_constants::AppConstants,
    app_model::{tile, EguiApp},
};

pub fn get_random_letter() -> char {
    let mut rng = rand::thread_rng();
    char::from_u32(rng.gen_range(65..=90)).unwrap()
}

pub fn get_random_color() -> Color32 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    egui::Color32::from_rgb(r, g, b)
}

pub fn get_contrast_label_color(background_color: Color32) -> Color32 {
    if background_color.r() > 200 || background_color.g() > 200 || background_color.b() > 200 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}
