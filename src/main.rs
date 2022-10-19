use std::collections::VecDeque;

use egui::RichText;
use egui_macroquad;
use macroquad::prelude::*;

use pathviz::*;

// use egui::Window;

#[macroquad::main("egui with macroquad")]
async fn main() {
    let r = SCREEN_WIDTH / RECT_WIDTH;
    let c = SCREEN_HEIGHT / RECT_WIDTH;
    let mut grid: Vec<Vec<pathviz::Rect>> = Vec::new();
    for i in 0..r as usize {
        let mut col: Vec<pathviz::Rect> = vec![];
        for j in 0..c as usize {
            let rect = pathviz::Rect::new(
                i as f32 * RECT_WIDTH,
                (j as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT,
                RECT_WIDTH,
                RECT_WIDTH,
                NONVIS_COLOR,
            );
            col.push(rect);
        }
        grid.push(col);
    }
    let mut st: VecDeque<(usize, usize)> = VecDeque::new();
    st.push_back((0, 0));

    let mut explorer = Explorer::new(
        grid,
        Some((0, 0)),
        Some(((r - 1.0) as usize, (c - 1.0) as usize)),
        st,
    );

    let mut game_state = State::Paused;
    let mut selected_algo = Algorithm::Dfs;
    let mut selected_speed = Speed::Average;
    let mut cur_button = MyButton::Wall;
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;

    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            x_pos = x;
            y_pos = y;
            match cur_button {
                MyButton::Start => {
                    if y >= TOP_PANEL_HEIGHT {
                        explorer.mark((x, y), START_COLOR)
                    }
                }
                MyButton::Target => {
                    if y >= TOP_PANEL_HEIGHT {
                        explorer.mark((x, y), TARGET_COLOR)
                    }
                }
                MyButton::Wall => {
                    if y >= TOP_PANEL_HEIGHT {
                        explorer.mark((x, y), WALL_COLOR)
                    }
                }
                MyButton::Reset => {
                    if y >= TOP_PANEL_HEIGHT {
                        explorer.reset();
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            let (x, y) = mouse_position();
            explorer.mark((x, y), NONVIS_COLOR)
        }

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
                            cur_button = MyButton::Start;
                        }
                        if ui
                            .button(RichText::new("Target Node").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            cur_button = MyButton::Target;
                        }
                        if ui
                            .button(RichText::new("Draw Walls").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            cur_button = MyButton::Wall;
                        }
                        if ui
                            .button(RichText::new("Play").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            game_state = State::Playing;
                        }
                        if ui
                            .button(RichText::new("Pause").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            game_state = State::Paused;
                        }

                        if ui
                            .button(RichText::new("Reset").size(BUTTON_WIDTH))
                            .clicked()
                        {
                            explorer.reset();
                            game_state = State::Paused;
                            cur_button = MyButton::Reset;
                        }
                        egui::ComboBox::from_label(RichText::new("Speed").size(20.0))
                            .selected_text(format!("{:?}", selected_speed))
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut selected_speed,
                                    Speed::Slow,
                                    RichText::new("Slow").size(25.0),
                                );
                                ui.selectable_value(
                                    &mut selected_speed,
                                    Speed::Average,
                                    RichText::new("Average").size(25.0),
                                );
                                ui.selectable_value(
                                    &mut selected_speed,
                                    Speed::Fast,
                                    RichText::new("Fast").size(25.0),
                                );
                            });
                        egui::ComboBox::from_label("Algorithms")
                            .selected_text(format!("{:?}", selected_algo))
                            .width(30.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut selected_algo, Algorithm::Dfs, "DFS");
                                ui.selectable_value(&mut selected_algo, Algorithm::Bfs, "BFS");
                            });
                    })
                });
            egui::Window::new("Debug window")
                .resizable(true)
                .show(egui_ctx, |ui| {
                    ui.label(format!("fps: {}", macroquad::time::get_fps()));
                    ui.label(format!("x: {}", x_pos));
                    ui.label(format!("y: {}", y_pos));
                    ui.label(format!("start: {:?}", explorer.start.unwrap()));
                    ui.label(format!("target: {:?}", explorer.target.unwrap()));
                    // ui.label(format!("last: {:?}", solver.last.back()));
                    ui.label(format!("currrent button: {:?}", cur_button));
                    ui.label(format!("selected algo: {:?}", selected_algo));
                    ui.label(format!("selected speed: {:?}", selected_speed));
                });
        });

        // Draw things before egui

        explorer.draw(&selected_algo, &mut game_state);

        egui_macroquad::draw();

        // Draw things after egui
        next_frame().await;
    }
}
