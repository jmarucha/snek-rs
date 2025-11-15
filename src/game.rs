
use rand::random_range;
use crate::clock::Clock;
use crate::snake::Snake;
use crate::Render;
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{KeyCode};

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

pub struct Game {
    snake: Snake,
    board_size: (u16, u16),
    render: Render,
    clock: Clock,
    previous_direction: Direction,
    direction: Direction,
    pub game_over: bool,
    food_pos: (u16, u16),
    score: u16
}

impl Game {
    pub fn new(w: u16, h: u16, render: Render) -> Game {
        Game {
            snake: Snake::new((10,10), 4),
            board_size: (w, h),
            render,
            clock: Clock::new(30.),
            previous_direction: Direction::None,
            direction: Direction::None,
            game_over: false,
            food_pos: (10, 15),
            score: 0,
        }
    }
    pub fn start(&mut self) {
        self.draw_food();
        self.draw_snake_segment(*self.snake.get_first_segment());
        self.render.flip().expect("Gówno");
    }

    pub fn run(&mut self) {
        if self.clock.rdy() {
            self.on_clock();
            self.clock.ack();
        }
    }

    fn clear_field(&mut self, pos: (u16, u16)) {
        self.render.draw_at(GraphicsObject::Empty, pos.0, pos.1).expect("Gówno");
    }
    fn draw_snake_segment(&mut self, pos: (u16, u16)) {
        self.render.draw_at(GraphicsObject::SnakeSegment, pos.0, pos.1).expect("Gówno");
    }

    fn draw_food(&mut self) {
        self.render.draw_at(GraphicsObject::Food, self.food_pos.0, self.food_pos.1).expect("Gówno");
    }

    fn on_clock(&mut self) {
        self.try_move_player();
        self.render.write_header(format!("Score: {}", self.score), 10).expect("Gówno");
    }


    fn try_move_player(&mut self) {
        let head_pos = self.snake.get_first_segment();
        let new_pos = match self.direction {
            Direction::Up => (head_pos.0, head_pos.1-1),
            Direction::Down => (head_pos.0, head_pos.1+1),
            Direction::Left => (head_pos.0-1, head_pos.1),
            Direction::Right => (head_pos.0+1, head_pos.1),
            Direction::None => return,
        };


        self.previous_direction = self.direction.clone();
        self.process_new_head_pos(new_pos)
    }

    fn generate_new_food(&mut self) {
        let new_pos = (random_range(1..self.board_size.0-1), random_range(1..self.board_size.1-1));
        self.food_pos = new_pos;
        self.draw_food()
    }

    fn process_new_head_pos(&mut self, new_pos: (u16, u16)) {
        if new_pos.0 < 1 || new_pos.0 >= self.board_size.0-1 ||
            new_pos.1 < 1 || new_pos.1 >= self.board_size.1-1 ||
            self.snake.is_inside(&new_pos) {
            self.game_over = true;
            return
        }

        self.draw_snake_segment(new_pos);
        self.snake.add_segment(new_pos);

        if new_pos == self.food_pos {
            self.score += 1;
            self.generate_new_food();
        } else {
            let tail = *self.snake.get_last_segment();
            self.clear_field(tail);
            self.snake.remove_last_segment();
        }
    }


    pub fn process_keystroke(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('w') => self.set_direction(Direction::Up),
            KeyCode::Down | KeyCode::Char('s') => self.set_direction(Direction::Down),
            KeyCode::Left | KeyCode::Char('a') => self.set_direction(Direction::Left),
            KeyCode::Right | KeyCode::Char('d') =>  self.set_direction(Direction::Right),
            _ =>{},
        }
    }

    fn set_direction(&mut self, new_direction: Direction) {
        self.direction = match (&self.previous_direction, &new_direction) {
            (Direction::Up, Direction::Down) => return,
            (Direction::Down, Direction::Up) => return,
            (Direction::Left, Direction::Right) => return,
            (Direction::Right, Direction::Left) => return,
            _ => new_direction
        }

    }

    pub fn end(&mut self) {
        sleep(Duration::from_secs(1));
        self.render.cleanup_screen();
        println!("\
+-----------------------+
| You scored{:3} points. |
+-----------------------+", self.score);
    }
}

pub enum GraphicsObject {
    SnakeSegment,
    Food,
    Empty
}