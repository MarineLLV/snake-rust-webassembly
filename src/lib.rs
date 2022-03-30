use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Directions
#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

// Create snake
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake{
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Down
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
            snake: Snake::new(snake_index) // start index 10
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

    // update the position
    pub fn update(&mut self) {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;
        let col = snake_index % self.width;

        if self.snake.direction == Direction::Right {
            let next_col = (col + 1) % self.width;
            self.snake.body[0].0 = (row * self.width) + next_col;
        }
        if self.snake.direction == Direction::Left {
            let next_col = (col - 1) % self.width;
            self.snake.body[0].0 = (row * self.width) + next_col;
        }
        if self.snake.direction == Direction::Up {
            let next_row = (row - 1) % self.width;
            self.snake.body[0].0 = (next_row * self.width) + col;
        }
        if self.snake.direction == Direction::Down {
            let next_row = (row + 1) % self.width;
            self.snake.body[0].0 = (next_row * self.width) + col;
        }

    }
}

// wasm-pack build --target web