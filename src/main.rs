use egui::RichText;
use macroquad::prelude::*;
use egui_macroquad;

use pathviz::*;

// use egui::Window;


#[macroquad::main("egui with macroquad")]
async fn main() {
    let c = SCREEN_WIDTH / RECT_WIDTH;
    let r = SCREEN_HEIGHT / RECT_WIDTH;
    let mut grid : Vec<Vec<pathviz::Rect>> = vec![vec![]];
    for i in 0..=c as usize {
        let mut col : Vec<pathviz::Rect> = vec![];
        for j in 0..=r as usize {
            let rect = pathviz::Rect::new(i as f32 * RECT_WIDTH , (j as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT, RECT_WIDTH, RECT_WIDTH, WHITE);
            col.push(rect);
        }
        grid.push(col);
    }

    let mut solver = Solver::new(grid, (0, 0), (50, 50));


    let mut selected_algo = Algorithm::Dfs;
    let mut selected_speed = Speed::Average;
    let mut cur_button = MyButton::Start;
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
                MyButton::Start => if y >= TOP_PANEL_HEIGHT { solver.mark((x, y), START_COLOR) },
                MyButton::Target => if y >= TOP_PANEL_HEIGHT { solver.mark((x, y), TARGET_COLOR) },
                MyButton::Wall => if y >= TOP_PANEL_HEIGHT { solver.mark((x, y), WALL_COLOR) },
            }
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("Demo panel").resizable(true).min_height(TOP_PANEL_HEIGHT).show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("Start Node").size(BUTTON_WIDTH)).clicked() {
                        cur_button = MyButton::Start;
                    }
                    if ui.button(RichText::new("Target Node").size(BUTTON_WIDTH)).clicked() {
                        cur_button = MyButton::Target;
                    }
                    if ui.button(RichText::new("Draw Walls").size(BUTTON_WIDTH)).clicked() {
                        cur_button = MyButton::Wall;
                    }
                    if ui.button(RichText::new("Play").size(BUTTON_WIDTH)).clicked() {
                        match selected_algo {
                            Algorithm::Dfs => solver.dfs(),
                            Algorithm::Bfs => solver.bfs(),
                        }
                    }
                    if ui.button(RichText::new("Pause").size(BUTTON_WIDTH)).clicked() {
                    }
                    egui::ComboBox::from_label(RichText::new("Speed").size(20.0))
                        .selected_text(format!("{:?}", selected_speed)).width(20.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected_speed, Speed::Slow, RichText::new("Slow").size(20.0));
                            ui.selectable_value(&mut selected_speed, Speed::Average, RichText::new("Average").size(20.0));
                            ui.selectable_value(&mut selected_speed, Speed::Fast, RichText::new("Fast").size(20.0));
                        }
                    );
                    egui::ComboBox::from_label("Algorithms")
                        .selected_text(format!("{:?}", selected_algo))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected_algo, Algorithm::Dfs, "DFS");
                            ui.selectable_value(&mut selected_algo, Algorithm::Bfs, "BFS");
                        }
                    );
                })
            });
            egui::Window::new("Debug window").resizable(true).show(egui_ctx, |ui| {
                ui.label(format!("x: {}", x_pos));
                ui.label(format!("y: {}", y_pos));
                ui.label(format!("currrent button: {:?}", cur_button));
                ui.label(format!("selected algo: {:?}", selected_algo));
                ui.label(format!("selected speed: {:?}", selected_speed));
            });
        });

        // Draw things before egui


        for row in solver.grid.iter() {
            for rect in row.iter() {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect.color);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GOLD);
            }
        }
        egui_macroquad::draw();



        
        // Draw things after egui

        next_frame().await;


    }
}

