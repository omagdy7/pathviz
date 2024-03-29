#![allow(unused_imports)]

use egui::RichText;
use egui_macroquad;
use macroquad::{color::colors, prelude::*};
use std::{
    collections::{HashMap, VecDeque},
    thread,
    time::Duration,
};

mod pathfinder;
mod pathfinderui;
pub use pathfinder::*;
pub use pathfinderui::PathUi;

pub const START_COLOR: Color = DARKBLUE;
pub const TARGET_COLOR: Color = RED;
pub const WALL_COLOR: Color = YELLOW;
pub const VIS_COLOR: Color = PURPLE;
pub const NONVIS_COLOR: Color = DARKGRAY;
pub const TRAIL_COLOR: Color = ORANGE;
pub const BUTTON_WIDTH: f32 = 40.0;
pub const TOP_PANEL_HEIGHT: f32 = 60.0;
pub const RECT_WIDTH: f32 = 30.0;
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0 - TOP_PANEL_HEIGHT;

pub type P2 = (usize, usize);
pub type Grid = Vec<Vec<Node>>;

pub trait PathFinder {
    fn explore(exp: &mut Explorer, state: &mut crate::State);
}

pub trait Render {
    fn render(
        explorer: &mut Explorer,
        cur_button: &mut MyButton,
        game_state: &mut State,
        selected_algo: &mut Algorithm,
        selected_speed: &mut Speed,
    );
}

pub struct Node {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub weight: u32,
    pub color: Color,
}

pub struct Explorer {
    pub grid: Grid,
    pub start: Option<P2>,
    pub target: Option<P2>,
    pub path: HashMap<P2, P2>,
    pub last: VecDeque<P2>,
}

impl Node {
    pub fn new(x: f32, y: f32, w: f32, h: f32, weight: u32, color: Color) -> Self {
        Node {
            x,
            y,
            w,
            h,
            weight,
            color,
        }
    }
}

impl Explorer {
    pub fn new(
        grid: Grid,
        start: Option<P2>,
        target: Option<P2>,
        path: HashMap<P2, P2>,
        last: VecDeque<P2>,
    ) -> Self {
        Self {
            grid,
            start,
            target,
            path,
            last,
        }
    }

    /* function to reset the playground */
    pub fn reset(&mut self) {
        /* reset all colors except the start and target nodes */
        for row in self.grid.iter_mut() {
            for rect in row.iter_mut() {
                if rect.color != START_COLOR || rect.color != TARGET_COLOR {
                    rect.color = NONVIS_COLOR;
                }
            }
        }
        self.last.clear(); /* empty stack */
        self.last.push_back(self.start.unwrap());
        let start = self.start.expect("Start was none");
        let target = self.target.expect("target was none");
        self.grid[start.0][start.1].color = START_COLOR;
        self.grid[target.0][target.1].color = TARGET_COLOR;
    }

    /* reset start node */
    fn reset_start(&mut self) {
        if let Some(st) = self.start {
            self.grid[st.0][st.1].color = NONVIS_COLOR;
            self.start = None;
        }
    }

    /* reset target node */
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
        if color == WALL_COLOR {
            self.grid[r][c].color.r -= 5.0;
        } else {
            self.grid[r][c].color = color;
        }
    }

    pub fn draw(&mut self, selected_algo: &Algorithm, game_state: &mut State) {
        match game_state {
            State::Playing => {
                if self.last.back().is_some() {
                    match selected_algo {
                        Algorithm::PathFinder(algo) => match algo {
                            PathFindingAlgorithm::Dfs => Dfs::explore(self, game_state),
                            PathFindingAlgorithm::Bfs => Bfs::explore(self, game_state),
                        },
                        _ => {}
                    }
                }
            }
            State::TargetFound => {
                self.mark_trail();
                self.grid[self.target.unwrap().0][self.target.unwrap().1].color = TARGET_COLOR;
            }
            _ => {}
        }
        for row in self.grid.iter() {
            for rect in row.iter() {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, rect.color);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
            }
        }
    }

    pub fn mark_trail(&mut self) {
        let mut cur = self.path.get(&self.target.unwrap());
        while cur.expect("cur shouldn't be none") != &self.start.expect("Start shouldn't be none") {
            self.grid[cur.expect("cur shouldn't be none").0]
                [cur.expect("cur shouldn't be none").1]
                .color = ORANGE;
            cur = self.path.get(cur.expect("cur shouldn't be none"));
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SortingAlgorithm {
    Insertion,
    Bubble,
    MergeSort,
    QuickSort,
}

#[derive(PartialEq, Debug)]
pub enum PathFindingAlgorithm {
    Dfs,
    Bfs,
}

#[derive(PartialEq, Debug)]
pub enum Algorithm {
    PathFinder(PathFindingAlgorithm),
    Sorter(SortingAlgorithm),
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
