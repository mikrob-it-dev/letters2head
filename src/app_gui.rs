use std::process::ExitCode;

use crate::app_constants::AppConstants;
use crate::app_model::{tile, EguiApp};
use crate::data_model::{self, KeyboardInstruction, StepResult};
use crate::utils::{get_contrast_label_color, get_random_color};
// use crate::{log_utils, utils};
use egui::FontFamily::Proportional;
use egui::{Align, Align2, Context, FontId, Label, Layout, Ui, Vec2, Widget};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let display_result_evaluation_window = true;

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
            // // egui::Window::new(AppConstants::APP_NAME)
            //     .fixed_pos(Pos2::new(10.0, 10.0))
            //     .collapsible(false)
            //     .auto_sized()
            //     .default_width(500.0)
            //     .show(ctx, |ui| {

            // keyboard input
            // let keyboard_instruction = register_keyboard_instructions(self, ui);

            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::exact(tile_size))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        // let mut button_grid = Vec::<egui::Button>::from([]);

                        let mut i = 0;
                        for x in 0..10 {
                            ui.horizontal(|ui| {
                                for y in 0..10 {
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

            ui_add_controls(ui, self, ctx, tile_size);

            if self.display_attempt_result || self.gui_initialize {
                ui_show_results_evaluation(ui, self, ctx, tile_size);
                
                // added to allow loading of the result window (hack, prevents slow response on first attempt)
                self.gui_initialize = false;
            }
        });

        // ui_add_license_info(self, ctx);
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
                // if license_button_handle.clicked() {
                //     my_app.is_license_info_shown = !my_app.is_license_info_shown;
                // }
            })
        }
    });
}

fn ui_add_controls(ui: &mut Ui, my_app: &mut EguiApp, ctx: &egui::Context, tile_size: f32) {
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
    ui: &mut Ui,
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

// fn ui_add_license_info(my_app: &mut EguiApp, ctx: &Context) {
//     egui::Window::new("License")
//         .collapsible(false)
//         .open(&mut my_app.is_license_info_shown)
//         .anchor(Align2::LEFT_TOP, [10.0, 10.0])
//         .min_width(800.0)
//         .min_height(800.0)
//         .vscroll(true)
//         .resizable(false)
//         .show(ctx, |ui| {
//             ui.add(Label::new(AppConstants::LICENSE_TEXT).wrap(true));
//         });
// }

fn calculate_row_height(text: &String, comment: &String) -> f32 {
    let step_number_of_lines: f32 = (text.lines().count() + comment.lines().count()) as f32;
    step_number_of_lines * AppConstants::FONT_SIZE * 1.2 // calibrated constant, ugly
}

// fn register_keyboard_instructions(egui_app: &mut EguiApp, ui: &mut Ui) -> KeyboardInstruction {
//     let events = ui.input().events.clone();
//     for event in &events {
//         match event {
//             egui::Event::Key {
//                 key,
//                 pressed,
//                 modifiers: _,
//             } => {
//                 if key == &egui::Key::Space && *pressed {
//                     if egui_app.checklist_position.step != 0 {
//                         if egui_app.selected_checklist.sections
//                             [egui_app.checklist_position.section - 1]
//                             .checklist_steps[egui_app.checklist_position.step - 1]
//                             .result
//                             == StepResult::Unattempted
//                         {
//                             return KeyboardInstruction::SkipStep;
//                         } else {
//                             return KeyboardInstruction::StepAhead;
//                         }
//                     } else {
//                         return KeyboardInstruction::StartSection;
//                     }
//                 }

//                 if key == &egui::Key::W && *pressed {
//                     return KeyboardInstruction::SkipSection;
//                 }
//                 if key == &egui::Key::A && *pressed {
//                     return KeyboardInstruction::RegisterOkResult;
//                 }
//                 if key == &egui::Key::D && *pressed {
//                     return KeyboardInstruction::RegisterNokResult;
//                 }
//             }
//             _ => {
//                 return KeyboardInstruction::None;
//             }
//         }
//     }

//     return KeyboardInstruction::None;
// }
