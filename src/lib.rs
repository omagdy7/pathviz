#![allow(unused_imports)]

use std::{collections::VecDeque, thread, time::Duration};
use egui::{color::HsvaGamma, RichText};
use egui_macroquad;
use macroquad::{color, prelude::*};
mod pathfinder;
pub use pathfinder::*;

pub const START_COLOR: Color = color::colors::DARKBLUE;
pub const TARGET_COLOR: Color = color::colors::RED;
pub const WALL_COLOR: Color = color::colors::YELLOW;
pub const VIS_COLOR: Color = color::colors::PURPLE;
pub const NONVIS_COLOR: Color = color::colors::DARKGRAY;
pub const BUTTON_WIDTH: f32 = 40.0;
pub const TOP_PANEL_HEIGHT: f32 = 60.0;
pub const RECT_WIDTH: f32 = 30.0;
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0 - TOP_PANEL_HEIGHT;

pub trait PathFinder {
    fn explore(exp: &mut Explorer, state: &mut crate::State);
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

pub struct Explorer {
    pub grid: Vec<Vec<Rect>>,
    pub start: Option<(usize, usize)>,
    pub target: Option<(usize, usize)>,
    pub last: VecDeque<(usize, usize)>,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rect { x, y, w, h, color }
    }
}

impl Explorer {
    pub fn new(
        grid: Vec<Vec<Rect>>,
        start: Option<(usize, usize)>,
        target: Option<(usize, usize)>,
        last: VecDeque<(usize, usize)>,
    ) -> Self {
        Self {
            grid,
            start,
            target,
            last,
        }
    }

    pub fn reset(&mut self) {
        for row in self.grid.iter_mut() {
            for rect in row.iter_mut() {
                if rect.color == WALL_COLOR || rect.color == VIS_COLOR {
                    rect.color = NONVIS_COLOR;
                }
            }
        }
        self.last.clear();
        self.last.push_back(self.start.unwrap());
        let start = self.start.unwrap();
        let target = self.target.unwrap();
        self.grid[start.0][start.1].color = START_COLOR;
        self.grid[target.0][target.1].color = TARGET_COLOR;
    }

    fn reset_start(&mut self) {
        if let Some(st) = self.start {
            self.grid[st.0][st.1].color = NONVIS_COLOR;
            self.start = None;
        }
    }

    fn reset_target(&mut self) {
        if let Some(end) = self.target {
            self.grid[end.0][end.1].color = NONVIS_COLOR;
            self.target = None;
        }
    }

    pub fn mark(&mut self, (x, y): (f32, f32), color: Color) {
        let r = (x / RECT_WIDTH) as usize;
        let c = ((y - TOP_PANEL_HEIGHT) / RECT_WIDTH) as usize;
        if color == START_COLOR {
            self.reset_start();
            self.start = Some((r, c));
            self.last.clear();
            self.last.push_back((r, c));
        } else if color == TARGET_COLOR {
            self.reset_target();
            self.target = Some((r, c));
        }

        self.grid[r][c].color = color;
    }

    pub fn draw(&mut self, selected_algo: &Algorithm, game_state: &mut State) {
        if let State::Playing = game_state {
            if self.last.back().is_some() {
                match selected_algo {
                    Algorithm::Dfs => Dfs::explore(self, game_state),
                    Algorithm::Bfs => Bfs::explore(self, game_state),
                }
            }
        }
        for row in self.grid.iter() {
            for rect in row.iter() {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect.color);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
            }
        }
    }
}

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
    Reset,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Playing,
    Paused,
    TargetFound,
}


pub fn is_valid_idx(i: i32, j: i32, r: i32, c: i32) -> bool {
    i >= 0 && j >= 0 && i < r && j < c
}
