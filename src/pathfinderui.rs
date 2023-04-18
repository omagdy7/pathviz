use crate::*;
use crate::{Algorithm, PathFindingAlgorithm, SortingAlgorithm, Speed, State};
use egui_macroquad;

pub struct PathUi {}

impl Render for PathUi {
    fn render(
        explorer: &mut Explorer,
        cur_button: &mut MyButton,
        game_state: &mut State,
        selected_algo: &mut Algorithm,
        selected_speed: &mut Speed,
    ) {
        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("Demo panel")
                .resizable(true)
                .min_height(TOP_PANEL_HEIGHT)
                .show(egui_ctx, |ui| {
                    ui.horizontal_centered(|ui| {
                        if ui
                            .button(RichText::new("Start Node").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            *cur_button = MyButton::Start;
                        }
                        if ui
                            .button(RichText::new("Target Node").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            *cur_button = MyButton::Target;
                        }
                        if ui
                            .button(RichText::new("Draw Walls").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            *cur_button = MyButton::Wall;
                        }
                        if ui
                            .button(RichText::new("Play").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            *game_state = State::Playing;
                        }
                        if ui
                            .button(RichText::new("Pause").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            *game_state = State::Paused;
                        }

                        if ui
                            .button(RichText::new("Reset").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            explorer.reset();
                            *game_state = State::Paused;
                            *cur_button = MyButton::Reset;
                        }
                        egui::ComboBox::from_label(RichText::new("Speed").size(20.0))
                            .selected_text(format!("{:?}", selected_speed))
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    selected_speed,
                                    Speed::Slow,
                                    RichText::new("Slow").size(25.0),
                                );
                                ui.selectable_value(
                                    selected_speed,
                                    Speed::Average,
                                    RichText::new("Average").size(25.0),
                                );
                                ui.selectable_value(
                                    selected_speed,
                                    Speed::Fast,
                                    RichText::new("Fast").size(25.0),
                                );
                            });
                        egui::ComboBox::from_label("Algorithms")
                            .selected_text(format!("{:?}", selected_algo))
                            .width(30.0)
                            .show_ui(ui, |ui| match selected_algo {
                                Algorithm::PathFinder(path_finder) => {
                                    ui.selectable_value(
                                        path_finder,
                                        PathFindingAlgorithm::Dfs,
                                        "DFS",
                                    );
                                    ui.selectable_value(
                                        path_finder,
                                        PathFindingAlgorithm::Bfs,
                                        "BFS",
                                    );
                                }
                                _ => {}
                            });
                    })
                });
            egui::Window::new("Debug window")
                .resizable(true)
                .scroll2([true, false])
                .show(egui_ctx, |ui| {
                    ui.label(format!("fps: {}", macroquad::time::get_fps()));
                    ui.label(format!("start: {:?}", explorer.start.unwrap()));
                    ui.label(format!("target: {:?}", explorer.target.unwrap()));
                    ui.label(format!("currrent button: {:?}", cur_button));
                    ui.label(format!("state: {:?}", game_state));
                    ui.label(format!("selected algo: {:?}", selected_algo));
                    ui.label(format!("selected speed: {:?}", selected_speed));
                });
        });
    }
}
