extern crate rand;

use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Tree = 0,
    Burning = 1,
    Empty = 2
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[wasm_bindgen]
pub struct Forest {
    width: u32,
    height: u32,
    p: f64,
    f: f64,
    patches: Vec<State>,
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.patches.as_slice().chunks(self.width as usize) {
            for &patch in line {
                let symbol = match patch {
                  State::Tree => 'T',
                  State::Burning => 'B',
                  State::Empty => 'E',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Forest {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_patch(&self, row: u32, column: u32) -> State {
      self.patches[self.get_index(row, column)]
    }

    fn has_burning_neighbor(&self, row: u32, column: u32) -> bool {
        if column > 0 {
            // W
            if self.get_patch(row, column-1) == State::Burning {
              return true;
            }
            if row > 0 {
              // NW
              if self.get_patch(row-1, column-1) == State::Burning {
                return true;
              }
            }
            if row < self.height-1 {
              // SW
              if self.get_patch(row+1, column-1) == State::Burning {
                return true;
              }
            }
        }
        if column < self.width-1 {
            // E = forest[i+1][j];
            if self.get_patch(row, column+1) == State::Burning {
              return true;
            }
            if row > 0 {
                // NE = forest[i+1][j-1];
              if self.get_patch(row-1, column+1) == State::Burning {
                return true;
              }
            }
            if row < self.height-1 {
                // SE = forest[i+1][j+1];
              if self.get_patch(row+1, column+1) == State::Burning {
                return true;
              }
            }
        }
        if row < self.height-1 {
          // S = forest[i][j+1];
          if self.get_patch(row+1, column) == State::Burning {
            return true;
          }
        }
        if row > 0 {
            // N = forest[i][j-1];
          if self.get_patch(row-1, column) == State::Burning {
            return true;
          }
        }
        false
    }

    fn lightning(&self) -> bool {
      let y: f64 = rand::thread_rng().gen();
      y < self.f
    }

    fn regenerate(&self) -> bool {
      let y: f64 = rand::thread_rng().gen();
      y < self.p
    }

    pub fn tick(&mut self) {
      let mut next = self.patches.clone();

      for row in 0..self.height {
        for col in 0..self.width {
          let idx = self.get_index(row, col);
          let patch_state = self.patches[idx];

          match patch_state {
            State::Tree => {
              if self.has_burning_neighbor(row, col) || self.lightning() {
                next[idx] = State::Burning;
              } else {
                next[idx] = State::Tree;
              }
            }
            State::Burning => { next[idx] = State::Empty; }
            State::Empty => {
              if self.regenerate() {
                next[idx] = State::Tree;
              } else {
                next[idx] = State::Empty;
              }
            }
          }
        }
      }

      self.patches = next;
    }

    pub fn new(width: u32, height: u32, p: f64, f: f64) -> Forest {
        let patches = (0..width * height)
            .map(|_| State::Tree)
            .collect();

        Forest {
            width,
            height,
            p,
            f,
            patches,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self) -> String {
      self.to_string()
    }

    pub fn patches(&self) -> *const State {
        self.patches.as_ptr()
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    Ok(())
}

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {

    Ok(())
}