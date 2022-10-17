#![allow(unused_imports)]

use egui::RichText;
use macroquad::{prelude::*, color};
use egui_macroquad;


pub const START_COLOR: Color = color::colors::MAGENTA;
pub const TARGET_COLOR: Color = color::colors::RED;
pub const WALL_COLOR: Color = color::colors::YELLOW;
pub const VIS_COLOR: Color = color::colors::PURPLE;
pub const NONVIS_COLOR: Color = color::colors::WHITE;
pub const BUTTON_WIDTH: f32 = 30.0;
pub const TOP_PANEL_HEIGHT : f32 = 60.0;
pub const RECT_WIDTH: f32 = 20.0;
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0 - TOP_PANEL_HEIGHT;

#[derive(PartialEq, Debug)]
pub enum Algorithm {
    Dfs,
    Bfs,
}

#[derive(PartialEq, Debug)]
pub enum Speed {
    Slow,
    Average,
    Fast,
}

#[derive(PartialEq, Debug)]
pub enum MyButton {
    Start,
    Target,
    Wall,
}


pub struct Solver {
    pub grid: Vec<Vec<Rect>>,
    pub start: Option<(usize, usize)>,
    pub target: Option<(usize, usize)>,
}

impl Solver {
    pub fn new(grid : Vec<Vec<Rect>>, start: Option<(usize, usize)>, target: Option<(usize, usize)>) -> Self {
        Self {
            grid,
            start,
            target,
        }
    }

    pub fn dfs(&mut self) {
        todo!();
    }

    pub fn bfs(&mut self) {
        todo!();
    }

    pub fn mark(&mut self, (x, y) : (f32, f32), color: Color) {
        self.grid[(x / RECT_WIDTH) as usize + 1][((y - TOP_PANEL_HEIGHT) / RECT_WIDTH) as usize].color = color;
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

pub fn sdf() {
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rect {
            x,
            y,
            w,
            h,
            color
        }
    }
}

