use super::*;

pub struct Dfs;

pub struct Bfs;

impl PathFinder for Bfs {
    fn explore(exp: &mut Explorer, state: &mut crate::State) {
        let (r, c) = exp.last.front().expect("In bfs").to_owned();
        let m = exp.grid.len() as i32;
        let n = exp.grid[0].len() as i32;

        exp.last.pop_front();

        let dx: [i32; 4] = [0, 1, 0, -1];
        let dy: [i32; 4] = [-1, 0, 1, 0];

        for i in 0..4 {
            let nx: i32 = r as i32 + dx[i];
            let ny: i32 = c as i32 + dy[i];
            if is_valid_idx(nx, ny, m, n)
                && exp.grid[nx as usize][ny as usize].color == TARGET_COLOR
            {
                exp.last.clear();
                *state = State::TargetFound;
            }

            if is_valid_idx(nx, ny, m, n)
                && (exp.grid[nx as usize][ny as usize].color == NONVIS_COLOR || exp.grid[nx as usize][ny as usize].color == TARGET_COLOR)
            {
                exp.last.push_back((nx as usize, ny as usize));
                exp.grid[nx as usize][ny as usize].color = VIS_COLOR;
                exp.path.insert((nx as usize, ny as usize), (r, c));
            }
        }
    }
}

impl PathFinder for Dfs {
    fn explore(exp: &mut Explorer, state: &mut State) {
        let (r, c) = exp.last.back().expect("in dfs").to_owned();
        let m = exp.grid.len() as i32;
        let n = exp.grid[0].len() as i32;

        exp.grid[r][c].color = VIS_COLOR;
        exp.last.pop_back();

        let dx: [i32; 4] = [0, 1, 0, -1];
        let dy: [i32; 4] = [-1, 0, 1, 0];



        for i in 0..4 {
            let nx: i32 = r as i32 + dx[i];
            let ny: i32 = c as i32 + dy[i];

            if is_valid_idx(nx, ny, m, n)
                && (exp.grid[nx as usize][ny as usize].color == NONVIS_COLOR || exp.grid[nx as usize][ny as usize].color == TARGET_COLOR)
            {
                if exp.grid[nx as usize][ny as usize].color == TARGET_COLOR /*&& (dx[i] == -1 || dx[i] == 1)*/
                {
                    exp.last.clear();
                    *state = State::TargetFound;
                } else {
                    exp.last.push_back((nx as usize, ny as usize));
                }
            }
        }
    }
}
