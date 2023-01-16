use crate::app_constants::AppConstants;
use crate::app_model::{EguiApp};
use crate::utils::{get_contrast_label_color};
// use crate::{log_utils, utils};
use egui::FontFamily::Proportional;
use egui::{Align, Align2, Context, FontId, Label, Ui, Vec2};
use egui_extras::{Size, StripBuilder};

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let tile_size =
            (ctx.available_rect().height() - AppConstants::FONT_SIZE * 1.6 - 50.0) / (11.0);

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Heading,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
            (
                egui::TextStyle::Body,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
            (
                egui::TextStyle::Button,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
            (
                egui::TextStyle::Small,
                FontId::new(AppConstants::FONT_SIZE, Proportional),
            ),
        ]
        .into();

        ctx.set_style(style);

        egui::CentralPanel::default()
        .show(ctx, |ui| {

            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(tile_size))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        // let mut button_grid = Vec::<egui::Button>::from([]);

                        for x in 0..AppConstants::GRID_SIZE_X {
                            ui.horizontal(|ui| {
                                for y in 0..AppConstants::GRID_SIZE_Y {
                                    let current_tile = self.tiles.get(x + y * 10).unwrap();

                                    ui.visuals_mut().override_text_color =
                                        Some(get_contrast_label_color(current_tile.color));

                                    let button =
                                        egui::Button::new(format!("{}", current_tile.letter))
                                            .min_size(Vec2::from([tile_size, tile_size]))
                                            .fill(current_tile.color);

                                    let button_handle = ui.add(button);
                                    if button_handle.clicked() {
                                        self.total_attempts += 1;
                                        self.display_attempt_result = true;

                                        let target_tile = self
                                            .tiles
                                            .get(usize::try_from(self.target_tile).unwrap())
                                            .unwrap();

                                        if current_tile.color == target_tile.color
                                            && current_tile.letter.eq(&target_tile.letter)
                                        {
                                            self.last_attempt_result_correct = true;
                                            self.correct_attempts += 1;
                                        } else {
                                            self.last_attempt_result_correct = false;
                                        }

                                        self.target_tile = EguiApp::set_target_tile();
                                    }
                                }
                            });
                            ui.add_space(5.0);
                        }
                    });
                    strip.cell(|ui| {
                        ui_add_dev_version_info(ui, self);
                    });
                });

            ui_add_controls(self, ctx, tile_size);

            if self.display_attempt_result || self.gui_initialize {
                ui_show_results_evaluation(self, ctx, tile_size);
                
                // added to allow loading of the result window (hack, prevents slow response on first attempt)
                self.gui_initialize = false;
            }
        });

        ui_add_license_info(self, ctx);
    }
}

fn ui_add_dev_version_info(ui: &mut Ui, my_app: &mut EguiApp) {
    let layout = egui::Layout::right_to_left(Align::Center);
    ui.with_layout(layout, |ui| {
        {
            ui.horizontal(|ui| {
                ui.hyperlink_to(
                    format!("{}", AppConstants::APP_DEVELOPER),
                    AppConstants::APP_DEVELOPER_WEBSITE,
                );
                ui.label("developed by:");
                ui.label(format!("|  version: {}  |", AppConstants::APP_VERSION));
                let license_button_handle = ui.button("License: MIT");
                if license_button_handle.clicked() {
                    my_app.is_license_info_shown = !my_app.is_license_info_shown;
                }
            })
        }
    });
}

fn ui_add_controls(my_app: &mut EguiApp, ctx: &egui::Context, tile_size: f32) {
    let mut layout = egui::Layout::top_down_justified(Align::Center);
    layout.main_align = Align::Center;
    egui::Window::new("Controls")
        .anchor(Align2::RIGHT_TOP, [-10.0, 10.0])
        // .default_pos(egui::Pos2::new(700.0, 10.0))
        .collapsible(false)
        .default_width(tile_size * 4.0)
        .show(ctx, |ui| {
            ui.with_layout(layout, |ui| {
                ui.vertical(|ui| {

                    ui.label(format!("Correct: {}", my_app.correct_attempts));

                    ui.label(format!("Total: {}", my_app.total_attempts));

                    ui.horizontal(|ui| {
                        let start_over_button = egui::Button::new("Start over")
                            .min_size(Vec2::from([tile_size * 4.0, 0.0]));

                        let start_over_button_handle = ui.add(start_over_button);

                        if start_over_button_handle.clicked() {
                            my_app.reset();
                        };
                    });

                    ui.horizontal(|ui| {
                        let exit_button =
                            egui::Button::new("Exit").min_size(Vec2::from([tile_size * 4.0, 0.0]));

                        let exit_button_handle = ui.add(exit_button);

                        if exit_button_handle.clicked() {
                            std::process::exit(0);
                        };
                    });

                    ui.horizontal(|ui| {
                        let mut style = (*ctx.style()).clone();
                        style.text_styles = [(
                            egui::TextStyle::Button,
                            FontId::new(24.0 * 4.0, Proportional),
                        )]
                        .into();
                        ui.style_mut().text_styles = style.text_styles;

                        let target_tile = my_app
                            .tiles
                            .get(usize::try_from(my_app.target_tile).unwrap())
                            .unwrap();

                        ui.visuals_mut().override_text_color =
                            Some(get_contrast_label_color(target_tile.color));

                        let button = egui::Button::new(format!("{}", target_tile.letter))
                            .min_size(Vec2::from([tile_size * 4.0, tile_size * 4.0]))
                            .fill(target_tile.color);

                        let button_handle = ui.add(button);
                        if button_handle.clicked() {}
                    });
                });
            });
        });
}

fn ui_show_results_evaluation(
    my_app: &mut EguiApp,
    ctx: &egui::Context,
    tile_size: f32,
) {
    let mut layout = egui::Layout::top_down_justified(Align::Center);
    layout.main_align = Align::Center;
    egui::Window::new("Result")
        .anchor(Align2::LEFT_TOP, [0.0, 0.0])
        .collapsible(false)
        .default_width(tile_size * 4.0)
        .show(ctx, |ui| {
            ui.with_layout(layout, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let mut style = (*ctx.style()).clone();
                        style.text_styles = [(
                            egui::TextStyle::Button,
                            FontId::new(24.0 * 10.0, Proportional),
                        )]
                        .into();

                        ui.style_mut().text_styles = style.text_styles;

                        let result_button_fill_color = if my_app.last_attempt_result_correct {
                            egui::Color32::DARK_GREEN
                        } else {
                            egui::Color32::DARK_RED
                        };

                        let result_button_text = if my_app.last_attempt_result_correct {
                            ":-)"
                        } else {
                            ":'-("
                        };

                        let result_button = egui::Button::new(result_button_text)
                            .min_size(Vec2::from([tile_size * 12.0, tile_size * 10.5]))
                            .fill(result_button_fill_color);

                        let result_button_handle = ui.add(result_button);

                        if result_button_handle.clicked() {
                            my_app.display_attempt_result = false;
                        };
                    });
                });
            });
        });
}

fn ui_add_license_info(my_app: &mut EguiApp, ctx: &Context) {
    egui::Window::new("License")
        .collapsible(false)
        .open(&mut my_app.is_license_info_shown)
        .anchor(Align2::LEFT_TOP, [10.0, 10.0])
        .min_width(1000.0)
        .min_height(800.0)
        .vscroll(true)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add(Label::new(AppConstants::LICENSE_TEXT).wrap(true));
        });
}
