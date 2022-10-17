#![allow(unused_imports)]

use egui::RichText;
use macroquad::prelude::*;
use egui_macroquad;


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


pub struct Solver {
    pub grid: Vec<Vec<Rect>>,
}

impl Solver {
    pub fn new(grid : Vec<Vec<Rect>>) -> Self {
        Self {
            grid
        }
    }

    pub fn dfs(&mut self) {
        todo!();
    }

    pub fn bfs(&mut self) {
        todo!();
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

