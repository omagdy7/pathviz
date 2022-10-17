use egui::RichText;
use macroquad::prelude::*;
use egui_macroquad;
// use egui::Window;


const BUTTON_WIDTH: f32 = 30.0;
const TOP_PANEL_HEIGHT : f32 = 60.0;
const RECT_WIDTH: f32 = 20.0;
const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0 - TOP_PANEL_HEIGHT;

#[derive(PartialEq, Debug)]
enum Algorithm {
    Dfs,
    Bfs,
}

#[derive(PartialEq, Debug)]
enum Speed {
    Slow,
    Average,
    Fast,
}

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
}

impl Rect {
    fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rect {
            x,
            y,
            w,
            h,
            color
        }
    }
    
}


#[macroquad::main("egui with macroquad")]
async fn main() {
    let playground_width = screen_width();
    let playground_height = screen_height() - TOP_PANEL_HEIGHT;
    let c = SCREEN_WIDTH / RECT_WIDTH;
    let r = SCREEN_HEIGHT / RECT_WIDTH;
    println!("width : {} height: {}", playground_width, playground_height);
    let mut grid : Vec<Vec<Rect>> = vec![vec![]];

    for i in 0..=c as usize {
        let mut col : Vec<Rect> = vec![];
        for j in 0..=r as usize {
            let rect = Rect::new(i as f32 * RECT_WIDTH , (j as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT, RECT_WIDTH, RECT_WIDTH, WHITE);
            col.push(rect);
        }
        grid.push(col);
    }
    println!("grid rows: {}, grid columns: {}", grid.len(), grid[1].len());



    let mut selected_algo = Algorithm::Dfs;
    let mut selected_speed = Speed::Average;
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    loop {
        clear_background(BLACK);

        // Process keys, mouse etc.
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            grid[(x / RECT_WIDTH) as usize + 1][((y - TOP_PANEL_HEIGHT) / RECT_WIDTH) as usize].color = YELLOW;
            x_pos = x;
            y_pos = y;
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("Demo panel").resizable(true).min_height(TOP_PANEL_HEIGHT).show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("Start Node").size(BUTTON_WIDTH)).clicked() {
                    }
                    if ui.button(RichText::new("Target Node").size(BUTTON_WIDTH)).clicked() {
                    }
                    if ui.button(RichText::new("Play").size(BUTTON_WIDTH)).clicked() {
                    }
                    if ui.button(RichText::new("Pause").size(BUTTON_WIDTH)).clicked() {
                    }
                    if ui.button(RichText::new("Exit").size(BUTTON_WIDTH)).clicked() {
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
            });
        });

        // Draw things before egui


        for row in grid.iter() {
            for rect in row.iter() {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect.color);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, BLUE);
            }
        }
        egui_macroquad::draw();



        
        // Draw things after egui

        next_frame().await;


    }
}

