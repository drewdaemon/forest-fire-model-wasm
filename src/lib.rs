use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::Rng;
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
    p: f32,
    f: f32,
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

    fn has_burning_neighbor(&self, row: u32, column: u32) -> bool {
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                if self.patches[idx] == State::Burning {
                  return true;
                }
            }
        }
        false
    }

    fn lightning(&self) -> bool {
      // let mut rng = rand::thread_rng();
      // rng.gen_range(0.0, 1.0) < self.f
      true
    }

    fn regenerate(&self) -> bool {
      // let mut rng = rand::thread_rng();
      // rng.gen_range(0.0, 1.0) < self.p
      true
    }

    pub fn tick(&mut self) {
      let mut next = self.patches.clone();

      for row in 0..self.height {
        for col in 0..self.width {
          let idx = self.get_index(row, col);
          let patch_state = self.patches[idx];
          let burning_neighbor = self.has_burning_neighbor(row, col);
          let lightning = self.lightning();
          let regenerate = self.regenerate();


          let next_state = match (patch_state, burning_neighbor, lightning, regenerate) {
            (State::Tree, _, true, _) => State::Burning,
            (State::Tree, true, _, _) => State::Burning,
            (State::Burning, _, _, _) => State::Empty,
            (State::Empty, _, _, true) => State::Tree,
            (otherwise, _, _, _) => otherwise,
          };

          next[idx] = next_state;
        }
      }

      self.patches = next;
    }

    pub fn new(width: u32, height: u32, p: f32, f: f32) -> Forest {
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