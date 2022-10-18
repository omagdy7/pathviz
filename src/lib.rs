#![allow(unused_imports)]

use std::{thread, time::Duration, collections::VecDeque};

use egui::{RichText, color::HsvaGamma};
use egui_macroquad;
use macroquad::{color, prelude::*};

pub const START_COLOR: Color  = color::colors::MAGENTA;
pub const TARGET_COLOR: Color = color::colors::RED;
pub const WALL_COLOR: Color   = color::colors::YELLOW;
pub const VIS_COLOR: Color    = color::colors::PURPLE;
pub const NONVIS_COLOR: Color = color::colors::DARKGRAY;
pub const BUTTON_WIDTH: f32 = 30.0;
pub const TOP_PANEL_HEIGHT: f32 = 60.0;
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

#[derive(PartialEq, Debug)]
pub enum State {
    Playing,
    Paused,
    TargetFound,
}

pub struct Solver {
    pub grid: Vec<Vec<Rect>>,
    pub start: Option<(usize, usize)>,
    pub target: Option<(usize, usize)>,
    pub last: VecDeque<(usize, usize)>
}

impl Solver {
    pub fn new(
        grid: Vec<Vec<Rect>>,
        start: Option<(usize, usize)>,
        target: Option<(usize, usize)>,
        last: VecDeque<(usize, usize)>
    ) -> Self {
        Self {
            grid,
            start,
            target,
            last,
        }
    }

    pub fn set_last(&mut self, value: Option<(usize, usize)>) {
        self.last.push_back(value.unwrap());
    }

    pub fn dfs(&mut self, r: usize, c: usize, state: &mut State) {
        self.grid[r][c].color = VIS_COLOR;
        draw_rectangle(r as f32 * RECT_WIDTH,(c as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT, RECT_WIDTH, RECT_WIDTH, VIS_COLOR);
        // draw_rectangle_lines(r as f32 * RECT_WIDTH, (c as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT, RECT_WIDTH, RECT_WIDTH, 2.0, VIS_COLOR);
        // thread::sleep(Duration::from_millis(10));
        self.last.pop_back();


        let dx: [i32; 4] = [0, 1,  0, -1];
        let dy: [i32; 4] = [-1, 0, 1,  0];

        for i in 0..4 {
            let nx: i32 = r as i32 + dx[i];
            let ny: i32 = c as i32 + dy[i];
            if self.is_valid_idx(nx, ny) && self.grid[nx as usize][ny as usize].color == TARGET_COLOR {
                self.last.clear();
                *state = State::TargetFound;
            }

            if self.is_valid_idx(nx, ny)
                && self.grid[nx as usize][ny as usize].color == NONVIS_COLOR
            {
                self.set_last(Some((nx as usize, ny as usize)));
            }
        }
    }

    pub fn bfs(&mut self, r: usize, c: usize, state: &mut State) {
        self.grid[r][c].color = VIS_COLOR;
        draw_rectangle(r as f32 * RECT_WIDTH,(c as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT, RECT_WIDTH, RECT_WIDTH, VIS_COLOR);
        // draw_rectangle_lines(r as f32 * RECT_WIDTH, (c as f32 * RECT_WIDTH) + TOP_PANEL_HEIGHT, RECT_WIDTH, RECT_WIDTH, 2.0, VIS_COLOR);
        // thread::sleep(Duration::from_millis(10));

        self.last.pop_front();

        let dx: [i32; 4] = [0, 1,  0, -1];
        let dy: [i32; 4] = [-1, 0, 1,  0];

        for i in 0..4 {
            let nx: i32 = r as i32 + dx[i];
            let ny: i32 = c as i32 + dy[i];
            if self.is_valid_idx(nx, ny) && self.grid[nx as usize][ny as usize].color == TARGET_COLOR {
                self.last.clear();
                *state = State::TargetFound;
            }

            if self.is_valid_idx(nx, ny)
                && self.grid[nx as usize][ny as usize].color == NONVIS_COLOR
            {
                self.last.push_front((nx as usize, ny as usize));
            }
        }
    }

    pub fn mark(&mut self, (x, y): (f32, f32), color: Color) {
        let r = (x / RECT_WIDTH) as usize;
        let c = ((y - TOP_PANEL_HEIGHT) / RECT_WIDTH) as usize;
        if color == START_COLOR {
            self.reset_start();
            self.start = Some((r, c));
            self.last.clear();
            self.last.push_back((r, c))
        } else if color == TARGET_COLOR {
            self.reset_target();
            self.target = Some((r, c));
        }

        // if self.is_safe_to_mark((r, c), color) {
        self.grid[r][c].color = color;
        // }
    }

    fn is_valid_idx(&self, i: i32, j: i32) -> bool {
        i >= 0 && j >= 0 && i < self.grid.len() as i32 && j < self.grid[0].len() as i32
    }

    pub fn draw(&mut self, selected_algo: &Algorithm, game_state: &mut State) {
        if let State::Playing = game_state {
            let (x, y) = self.last.back().unwrap();
            let (xb, yb) = self.last.front().unwrap();
            match selected_algo {
                Algorithm::Dfs => self.dfs(*x, *y, game_state),
                Algorithm::Bfs => self.bfs(*xb, *yb, game_state),
            }
        }

        for row in self.grid.iter() {
            for rect in row.iter() {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect.color);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
            }
        }
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
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Self {
        Rect { x, y, w, h, color }
    }
}
