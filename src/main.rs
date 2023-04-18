use egui::RichText;
use egui_macroquad;
use macroquad::prelude::*;
use pathviz::*;
use std::collections::{HashMap, VecDeque};

mod pathfinder;
mod pathfinderui;
mod sorterui;
pub use pathfinder::*;
pub use pathfinderui::PathUi;
pub use sorterui::SorterUi;

#[macroquad::main("egui with macroquad")]
async fn main() {
    let r = SCREEN_WIDTH / RECT_WIDTH;
    let c = SCREEN_HEIGHT / RECT_WIDTH;
    let mut grid: Grid = Vec::new();
    for i in 0..r as usize {
        let mut col: Vec<pathviz::Node> = vec![];
        for j in 0..c as usize {
            let rect = pathviz::Node::new(
                i as f32 * RECT_WIDTH,
                (j as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT,
                RECT_WIDTH,
                RECT_WIDTH,
                0,
                NONVIS_COLOR,
            );
            col.push(rect);
        }
        grid.push(col);
    }
    let mut st: VecDeque<P2> = VecDeque::new();
    let trail = HashMap::new();
    st.push_back((0, 0));

    let mut explorer = Explorer::new(
        grid,
        Some((0, 0)),
        Some(((r - 1.0) as usize, (c - 1.0) as usize)),
        trail,
        st,
    );

    let mut game_state = State::Paused;
    let mut selected_algo = Algorithm::PathFinder(PathFindingAlgorithm::Dfs);
    let mut selected_speed = Speed::Average;
    let mut cur_button = MyButton::Wall;

    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
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

        // Draw things before egui
        PathUi::render(
            &mut explorer,
            &mut cur_button,
            &mut game_state,
            &mut selected_algo,
            &mut selected_speed,
        );

        // SorterUi::render(
        //     &mut explorer,
        //     &mut cur_button,
        //     &mut game_state,
        //     &mut selected_algo,
        //     &mut selected_speed,
        // );

        explorer.draw(&selected_algo, &mut game_state);

        egui_macroquad::draw();
        // Draw things after egui
        next_frame().await;
    }
}
