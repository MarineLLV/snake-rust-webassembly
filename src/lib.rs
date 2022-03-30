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

#[derive(Clone)]
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
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 { return; }

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
        let temp = self.snake.body.clone();
        let next_cell = self.generate_next_snake_cell(&self.snake.direction);
        self.snake.body[0] = next_cell;

        let length = self.snake.body.len();

        // Moving the cells --> starting from the body (not the head)
        for i in 1..length {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;

        return match direction {
            Direction::Right => {
                SnakeCell((row * self.width) + (snake_index + 1) % self.width())
            },
            Direction::Left => {
                SnakeCell((row * self.width) + (snake_index - 1) % self.width())
            },
            Direction::Up => {
                SnakeCell((snake_index - self.width) % self.size)
            },
            Direction::Down => {
                SnakeCell((snake_index + self.width) % self.size)
            },
        };
    }
}

// wasm-pack build --target web