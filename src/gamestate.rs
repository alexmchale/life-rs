use events::{EventStatus};
use sdl2::pixels::Color;
use sdl2::rect::{Point};
use sdl_engine::{SdlEngine};
use std::process::exit;

#[derive(Debug)]
pub struct GameState {
    map: Vec<bool>,
    width: usize,
    height: usize,
}

impl<'a> GameState {
    pub fn new(map_width: usize, map_height: usize) -> Self {
        let mut state = GameState{
            map: vec![],
            width: map_width,
            height: map_height,
        };

        for _ in 0..map_height {
            for _ in 0..map_width {
                state.map.push(false);
            }
        }

        state.set_cell(101, 105, true);
        state.set_cell(101, 106, true);

        state.set_cell(102, 105, true);
        state.set_cell(102, 106, true);

        state.set_cell(111, 105, true);
        state.set_cell(111, 106, true);
        state.set_cell(111, 107, true);

        state.set_cell(112, 104, true);
        state.set_cell(112, 108, true);

        state.set_cell(113, 103, true);
        state.set_cell(113, 109, true);

        state.set_cell(114, 103, true);
        state.set_cell(114, 109, true);

        state.set_cell(115, 106, true);

        state.set_cell(116, 104, true);
        state.set_cell(116, 108, true);

        state.set_cell(117, 105, true);
        state.set_cell(117, 106, true);
        state.set_cell(117, 107, true);

        state.set_cell(118, 106, true);

        state.set_cell(121, 103, true);
        state.set_cell(121, 104, true);
        state.set_cell(121, 105, true);

        state.set_cell(122, 103, true);
        state.set_cell(122, 104, true);
        state.set_cell(122, 105, true);

        state.set_cell(123, 102, true);
        state.set_cell(123, 106, true);

        state.set_cell(125, 101, true);
        state.set_cell(125, 102, true);
        state.set_cell(125, 106, true);
        state.set_cell(125, 107, true);

        state.set_cell(135, 103, true);
        state.set_cell(135, 104, true);

        state.set_cell(136, 103, true);
        state.set_cell(136, 104, true);

        state
    }
    pub fn get_cell(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 { return false };
        let xu = x as usize;
        let yu = y as usize;
        xu < self.width && yu < self.height && self.map[yu*self.width+xu]
    }
    pub fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        self.map[y*self.width+x] = alive;
    }
    pub fn update_events(self, status: &EventStatus) -> Self {
        if status.key_escape {
            exit(0);
        }

        self
    }
    pub fn next_state(self) -> Self {
        let mut state = GameState{
            map: Vec::with_capacity(self.map.capacity()),
            width: self.width,
            height: self.height,
        };

        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.get_cell(x as i32, y as i32);
                let living_neighbors = self.count_living_neighbors(x, y);

                state.map.push(
                    if alive && living_neighbors < 2 {
                        // Underpopulation / loneliness
                        false
                    } else if alive && living_neighbors > 3 {
                        // Overpopulation
                        false
                    } else if !alive && living_neighbors == 3 {
                        // Reproduction
                        true
                    } else {
                        // No change
                        alive
                    }
                );
            }
        }

        state
    }
    pub fn count_living_neighbors(&self, x: usize, y: usize) -> i8 {
        let mut count = 0;

        if self.get_cell(x as i32 - 1, y as i32 - 1) { count += 1 };
        if self.get_cell(x as i32    , y as i32 - 1) { count += 1 };
        if self.get_cell(x as i32 + 1, y as i32 - 1) { count += 1 };
        if self.get_cell(x as i32 - 1, y as i32    ) { count += 1 };
        if self.get_cell(x as i32 + 1, y as i32    ) { count += 1 };
        if self.get_cell(x as i32 - 1, y as i32 + 1) { count += 1 };
        if self.get_cell(x as i32    , y as i32 + 1) { count += 1 };
        if self.get_cell(x as i32 + 1, y as i32 + 1) { count += 1 };

        count
    }
    pub fn render(&mut self, sdl_engine: &mut SdlEngine, gfx_scale: usize) {
        let map_width = self.width;
        let map_height = self.height;

        sdl_engine.renderer().set_draw_color(Color::RGB(200, 100, 100));

        for y in 0..map_height {
            for x in 0..map_width {
                if self.map[y*map_width + x] {
                    for x1 in 0..gfx_scale {
                        for y1 in 0..gfx_scale {
                            sdl_engine.renderer().draw_point(Point::new(((x*gfx_scale)+x1) as i32, ((y*gfx_scale)+y1) as i32)).unwrap();
                        }
                    }
                }
            }
        }
    }
}
