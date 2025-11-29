use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw;
use draw::draw_block;


// green snake
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction, // Direction in which the snake is travelling
    body: LinkedList<Block>,
    tail: Option<Block>, // we want tail to be an actual value when we eat an apple
}

impl Snake {
    // intialize a default snake that is 3 blocks long and moving towards right
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        // In the start our snake will be 3 blocks long
        // Initializing all the 3 blocks vv
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y,
        });
        body.push_back(Block {
            x,
            y,
        });

        Snake {
            direction: Direction::Right, // initially, snake will be moving in the right direction
            body,
            tail: None,
        }
    }

    // draws the entire body of the snake
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }
    
    // fn to get the head position of the snake
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction{
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        // adding the new block to the front of the snake
        self.body.push_front(new_block);

        // removing the last block
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // return the direction the snake is moving in
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // calculating the next position of head based on the moving direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // if we eat an apple, tail will be pushed back to snake's body
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // if snake is going in circle
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;

        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}

