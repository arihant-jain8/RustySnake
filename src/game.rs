use piston_window::*;
use piston_window::types::Color;

use rand::Rng;

use crate::{draw, snake};
use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.3;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool, // if food already exists on the board, we wont be spawning more food
    food_x: i32,
    food_y: i32,
    
    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key){
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up    | Key::W => Some(Direction::Up),
            Key::Down  | Key::S => Some(Direction::Down),
            Key::Left  | Key::A => Some(Direction::Left),
            Key::Right | Key::D => Some(Direction::Right),
            _ => None,
        };

        // if pressed key is opposite to the direction the snake is moving in, we will return out of this fn
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // draw the snake
        self.snake.draw(con, g);

        // drawing the food if it exists
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // draw all the borders of the board
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width - 1, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        
        // draw a red box on the entire board
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);            
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // if the game is over, restarting the game from start
        if self.game_over{
            if self.waiting_time > RESTART_TIME {
                self.restart()
            }
            return;
        }

        // adding food on the board if no food present
        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // checking if the snake ate food block
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();

        // if the snake has eaten food, we restore the tail
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    // checks if snake hasnt overlapped itself or run out of boundary
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // returning false if the snake is overlapping itself
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // return true if the snake is within the border of the boards
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = rand::rng();

        let mut new_x = rng.random_range(1..(self.width - 1));
        let mut new_y = rng.random_range(1..(self.height - 1));

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.random_range(1..(self.width - 1));
            new_y = rng.random_range(1..(self.height - 1));
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    // fn to restart the game after a game over
    // we are not calling the new function because we dont want to load a new window everytime the game resets
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}