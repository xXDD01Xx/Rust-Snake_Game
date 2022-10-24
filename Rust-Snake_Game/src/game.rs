use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [1.00, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const GAME_OVER_COLOR: Color = [0.80, 0.00, 0.00, 0.50];

const MOVING_PERIOD:f64 = 0.15;
const RESTART_TIME:f64 = 1.50;

pub struct Game
{
    snake: Snake,

    food_exists:bool,
    food_x:i32,
    food_y:i32,

    board_width:i32,
    board_height:i32,

    game_over:bool,
    waiting_time:f64,
}

impl Game
{
    pub fn new_game(board_width:i32, board_height:i32) -> Game
    {
        Game
        {
            //TODO
            snake:Snake::new_snake(2,2),
            waiting_time: 0.00,
            food_exists: true,
            food_x: 8,
            food_y: 8,
            board_width,
            board_height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key)
    {
        if self.game_over
        {
            return;
        }

        let direction = match key
        {
          Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction()),
        };

        if let Some(direction) = direction
        {
            if direction == self.snake.head_direction().block_opposite_direction_move()
            {
                return;
            }
        }

        self.update_snake(direction);
    }

    pub fn draw(&self, context: &Context, graphics_buffer:&mut G2d)
    {
        self.snake.draw(context, graphics_buffer);

        if self.food_exists
        {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, graphics_buffer);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.board_width, 1, context, graphics_buffer);
        draw_rectangle(BORDER_COLOR, 0, self.board_height -1, self.board_width, 1, context, graphics_buffer);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.board_height, context, graphics_buffer);
        draw_rectangle(BORDER_COLOR, self.board_width -1, 0, 1, self.board_height, context, graphics_buffer);

        if self.game_over
        {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.board_width, self.board_height, context, graphics_buffer);
        }
    }

    pub fn update(&mut self, delta_time:f64)
    {
        self.waiting_time += delta_time;

        if self.game_over
        {
            if self.waiting_time > RESTART_TIME
            {
                self.restart();
            }
            return;
        }

        if !self.food_exists
        {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD
        {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self)
    {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y
        {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, direction: Option<Direction>) -> bool
    {
        let (next_x, next_y) = self.snake.next_head(direction);

        if self.snake.head_tail_overlap(next_x, next_y)
        {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.board_width -1 && next_y < self.board_height -1
    }

    fn add_food(&mut self)
    {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.board_width -1);
        let mut new_y = rng.gen_range(1..self.board_height -1);

        while self.snake.head_tail_overlap(new_x, new_y)
        {
            new_x = rng.gen_range(1..self.board_width -1);
            new_y = rng.gen_range(1..self.board_height -1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, direction: Option<Direction>)
    {
        if self.check_if_snake_alive(direction)
        {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self)
    { //TODO
        self.snake = Snake::new_snake(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 8;
        self.food_y = 8;
        self.game_over = false;
    }
}