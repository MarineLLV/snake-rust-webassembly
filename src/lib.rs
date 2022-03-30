use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Create snake
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake{
            body: vec!(SnakeCell(spawn_index))
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
    pub fn new() -> World {
        let width = 8;
        World {
            width,
            size: width * width,
            snake: Snake::new(10) // start index 10
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
        self.snake.body[0].0 = (snake_index + 1) % self.size; // to come back to square 0

    }
}

// wasm-pack build --target web