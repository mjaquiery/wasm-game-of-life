mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use fixedbitset::FixedBitSet;
use rand::Rng;

#[wasm_bindgen]
/// A struct representing the game of life.
pub struct Universe {
    /// Width of the universe
    width: u32,
    /// Height of the universe
    height: u32,
    /// The cells of the game.
    cells: FixedBitSet,
    /// The generation of the universe
    generation: u32,
}

#[wasm_bindgen]
impl Universe {

    /// Create a new universe with a given width, height and initial probability of a cell being alive.
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let universe = Universe::new(Some(10), Some(10), Some(0.5));
    /// assert_eq!(universe.width(), 10);
    /// assert_eq!(universe.height(), 10);
    /// assert!(universe.population() > 0);
    /// assert!(universe.population() < 100);
    /// assert_eq!(universe.generation(), 0);
    /// ```
    pub fn new(
        width: Option<u32>,
        height: Option<u32>,
        initial_probability: Option<f64>
    ) -> Universe {
        let width = width.unwrap_or(64);
        let height = height.unwrap_or(64);
        let initial_probability = initial_probability.unwrap_or(0.0);

        let mut cells = FixedBitSet::with_capacity((width * height) as usize);
        let mut rng = rand::thread_rng();

        for i in 0..cells.len() {
            cells.set(i, rng.gen::<f64>() < initial_probability)
        }

        Universe {
            width,
            height,
            cells,
            generation: 0,
        }
    }

    /// Get the width of the universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get the generation of the universe
    pub fn generation(&self) -> u32 { self.generation }

    /// Get the population of the universe
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let mut universe = Universe::new(Some(10), Some(10), None);
    /// universe.set_cell(1, 1, true);
    /// universe.set_cell(1, 2, true);
    /// assert_eq!(universe.population(), 2);
    /// ```
    pub fn population(&self) -> u32 {
        self.cells.count_ones(..) as u32
    }

    /// Set a specific cell in the universe
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let mut universe = Universe::new(Some(3), Some(3), None);
    /// universe.set_cell(1, 1, true);
    /// assert_eq!(
    ///     universe.get_cells(),
    ///     vec![0, 0, 0, 0, 1, 0, 0, 0, 0]
    /// );
    /// ```
    pub fn set_cell(&mut self, row: u32, column: u32, state: bool) {
        let index = self.get_index(row, column);
        self.cells.set(index, state);
    }

    /// Toggle a specific cell in the universe
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let mut universe = Universe::new(Some(3), Some(3), None);
    /// universe.set_cell(1, 1, true);
    /// universe.toggle_cell(1, 1);
    /// assert_eq!(
    ///    universe.get_cells(),
    ///    vec![0, 0, 0, 0, 0, 0, 0, 0, 0]
    /// );
    /// ```
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let index = self.get_index(row, column);
        self.cells.toggle(index);
    }

    /// Get the universe cells as Vec<u8>
    /// # Example
    /// ```
    /// use wasm_game_of_life::Universe;
    /// let mut universe = Universe::new(Some(3), Some(3), None);
    /// println!("{:?}", universe.get_cells());
    /// ```
    pub fn get_cells(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for i in 0..self.cells.len() {
            if self.cells[i] {
                vec.push(1);
            } else {
                vec.push(0);
            }
        }
        vec
    }

    /// Get the universe as a pointer for direct-access reading in JS
    /// # Example
    /// ```
    /// use wasm_game_of_life::Universe;
    /// let mut universe = Universe::new(Some(3), Some(3), Some(0.5));
    /// println!("{:?}", universe.get_cells_as_ptr());
    /// ```
    pub fn get_cells_as_ptr(&self) -> *const u32 {
        self.cells.as_slice().as_ptr() as *const u32
    }

    /// Set the cells of the universe
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let mut universe = Universe::new(Some(3), Some(3), None);
    /// universe.set_cells(vec![1; 3 * 3]);
    /// ```
    pub fn set_cells(&mut self, cells: Vec<u8>) {
        if cells.len() as u32 != self.width * self.height {
            panic!("Cell count does not match universe size.");
        }

        for (i, cell) in cells.iter().enumerate() {
            self.cells.set(i, *cell > 0);
        }
    }

    /// Get a cell vector index from row, column
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        // Add a whole row/column and take the modulous to support wrapping
        let row = (row + self.height) % self.height;
        let column = (column + self.width) % self.width;
        (row * self.width + column) as usize
    }

    /// Return the number of living neighbours for the cell at a given row, column
    fn count_living_neighbours(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        let deltas: [i32;3] = [-1, 0, 1];
        deltas.iter()
            .for_each(|dr| {
                deltas.iter()
                    .for_each(|dc| {
                        // The cell itself is not a neighbour
                        if *dr == 0 && *dc == 0 {
                            return;
                        }
                        let row_index = row as i32 + dr;
                        let column_index = column as i32 + dc;
                        count += self.cells[
                            self.get_index(row_index as u32, column_index as u32)
                            ] as u8;
                    })
            });
        count
    }

    /// Return the cell state depending on the number of living neighbours.
    /// The rules of the Game Of Life state that:
    /// 1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    /// 2. Any live cell with two or three live neighbours lives on to the next generation.
    /// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn get_next_cell_state(&self, living_neighbour_count: u8, current_state: bool) -> bool {
        match current_state {
            false => {
                if living_neighbour_count == 3 {
                    true
                } else {
                    false
                }
            },
            true => {
                if living_neighbour_count == 2 || living_neighbour_count == 3 {
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Run an update step for the Universe
    ///
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let mut universe = Universe::new(Some(5), Some(5), None);
    /// // Build a line
    /// universe.set_cells(vec![
    ///     0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0,
    ///     0, 1, 1, 1, 0,
    ///     0, 0, 0, 0, 0,
    ///     0, 0, 0, 0, 0,
    /// ]);
    ///
    /// println!("{}", universe);
    /// // Run a tick
    /// universe.tick();
    /// println!("{}", universe);
    ///
    /// // Check the state of the universe
    /// assert_eq!(
    ///    universe.get_cells(),
    ///     vec![
    ///         0, 0, 0, 0, 0,
    ///         0, 0, 1, 0, 0,
    ///         0, 0, 1, 0, 0,
    ///         0, 0, 1, 0, 0,
    ///         0, 0, 0, 0, 0,
    ///     ]
    /// );
    pub fn tick(&mut self) {
        let mut next= self.cells.clone();
        (1..self.height).into_iter()
            .for_each(
                |r| {
                    (1..self.width).into_iter()
                        .for_each(
                            |c| {
                                let index = self.get_index(r, c);
                                let neighbours_alive = self.count_living_neighbours(r, c);
                                next.set(
                                    index,
                                    self.get_next_cell_state(neighbours_alive, self.cells[index])
                                );
                            }
                        )
                }
            );
        self.cells = next;
        self.generation += 1;
    }
}

impl fmt::Display for Universe {
    /// Display the universe as a grid of cells
    /// # Example
    /// ```
    /// use wasm_game_of_life::{Universe};
    /// let universe = Universe::new(Some(10), Some(10), Some(0.5));
    /// println!("{}", universe);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let symbol = if self.cells[self.get_index(r, c)] { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
                if c == self.width - 1 {
                    write!(f, "\n")?;
                }
            }
        }

        Ok(())
    }
}
