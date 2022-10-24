use std::collections::LinkedList;
use std::sync::TryLockError::WouldBlock;
use piston_window::{Context, G2d};
use piston_window::types::Color;

// use draw::draw_block;
use crate::draw::draw_block;

#[derive(Clone, Debug)]
struct Block
{
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

impl Direction
{
    pub fn block_opposite_direction_move(&self) -> Direction
    {
        match *self
        {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

                            //red, blue, green, opacity
const SNAKE_COLOR: Color = [0.00, 1.00, 0.00, 1.0];

pub struct Snake
{
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake
{
    pub fn new_snake(x: i32, y: i32) -> Snake
    {
        let mut body: LinkedList<Block> = LinkedList::new();
        //push_back appends to the end of the list
        body.push_back
        (
            Block
            {
                x: x + 1,
                y,
            }
        );
        body.push_back
        (
            Block
            {
                x,
                y,
            }
        );

        Snake
        {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context, graphics_buffer: &mut G2d)
    {
        for block in &self.body
        {
            draw_block(SNAKE_COLOR, block.x, block.y, context, graphics_buffer);
        }
    }

    pub fn head_position(&self) -> (i32, i32)
    {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>)
    {
        match direction
        {
            Some(dir) => self.direction = dir,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();
        //removing the last block and adding new block in the concurrent direction
        let new_block = match self.direction
        {
            Direction::Up => Block
            {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block
            {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block
            {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block
            {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction
    {
        self.direction
    }

    pub fn next_head(&self, direction:Option<Direction>) -> (i32, i32)
    {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_direction = self.direction;
        match direction
        {
            Some(dir) => moving_direction = dir,
            None => {}
        }

        match moving_direction
        {
            Direction::Up => (head_x, head_y -1),
            Direction::Down => (head_x, head_y +1),
            Direction::Left => (head_x -1, head_y),
            Direction::Right => (head_x +1, head_y),
        }
    }

    //if we eat an apple this method is run
    pub fn restore_tail(&mut self)
    {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn head_tail_overlap(&self, x:i32, y:i32) -> bool
    {
        let mut check = 0;
        for block in &self.body
        {
            if x == block.x && y == block.y
            {
                return true;
            }

            check +=1;
            if check == self.body.len() -1
            {
                break;
            }
        }
        return false;

    }
}




