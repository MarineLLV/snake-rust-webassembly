use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
// Directions
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

// Create snake
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake{
            body,
            direction: Direction::Right
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_index, 3) // start index 10
        }
    }

    // create getter for width
    pub fn width(&self) -> usize {
        self.width
    }

    // snake to the world
    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    // get the snake length
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const = raw pointer --> borrowing rules not applied
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    // can't return a reference to JS because of borrowing rules
    // get the snake cells
/*     pub fn snake_celles(&self) -> &Vec<SnakeCell> {
        &self.snake.body
    } */


    // update the position
    pub fn update(&mut self) {
        let snake_index = self.snake_head_index();

        let (row, col) = self.index_to_cell(snake_index);
        let (row, col) = match self.snake.direction {
            Direction::Right => {
                // return a tuple of 2 items : row and column
                (row, (col + 1) % self.width)
            },
            Direction::Left => {
                (row, (col - 1) % self.width)
            },
            Direction::Up => {
                ((row - 1) % self.width, col)
            },
            Direction::Down => {
                ((row + 1) % self.width, col)
            },
        };

        let next_index = self.cell_to_index(row, col);
        self.set_snake_head(next_index);
    }

    fn set_snake_head(&mut self, index: usize) {
        self.snake.body[0].0 = index;
    }

    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}

// wasm-pack build --target web