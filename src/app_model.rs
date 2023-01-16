use egui::Color32;
use rand::Rng;

use crate::{
    app_constants::AppConstants,
    utils::{get_random_color, get_random_letter},
};

pub struct EguiApp {
    pub tiles: Vec<Tile>,
    pub target_tile: u32,
    pub last_attempt_result_correct: bool,
    pub display_attempt_result: bool,
    pub correct_attempts: u32,
    pub total_attempts: u32,
    pub gui_initialize: bool,
    pub is_license_info_shown: bool,
}

impl EguiApp {
    pub fn default() -> Self {
        Self {
            tiles: Self::init_tiles(),
            target_tile: Self::set_target_tile(),
            last_attempt_result_correct: false,
            display_attempt_result: false,
            correct_attempts: 0,
            total_attempts: 0,
            gui_initialize: true,
            is_license_info_shown: false,
        }
    }

    pub fn reset(&mut self) {
        self.tiles = Self::init_tiles();
        self.target_tile = Self::set_target_tile();
    }

    pub fn set_target_tile() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..(AppConstants::GRID_SIZE_X * AppConstants::GRID_SIZE_Y)).try_into().unwrap()
    }

    pub fn init_tiles() -> Vec<Tile> {
        let mut tiles = Vec::<Tile>::from([]);

        for i in 0..(AppConstants::GRID_SIZE_X * AppConstants::GRID_SIZE_Y).try_into().unwrap() {
            let tile = Tile {
                id: i,
                letter: get_random_letter(),
                color: get_random_color(),
            };

            tiles.push(tile);
        }

        tiles
    }
}

pub struct Tile {
    pub id: u32,
    pub letter: char,
    pub color: Color32,
}
