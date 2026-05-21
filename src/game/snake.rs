use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Head,
    Body,
    Tail,
    Food,
}

pub struct SnakeGame {
    pub grid_width: i32,
    pub grid_height: i32,
    snake: VecDeque<Position>,
    direction: Direction,
    next_direction: Direction,
    food: Position,
    pub score: u32,
    pub game_over: bool,
    tick_accumulator: f64,
    pub tick_rate: f64,
}

impl SnakeGame {
    pub fn new(grid_width: i32, grid_height: i32) -> Self {
        let start = Position::new(grid_width / 2, grid_height / 2);
        let mut snake = VecDeque::new();
        snake.push_back(start);
        snake.push_back(Position::new(start.x - 1, start.y));
        snake.push_back(Position::new(start.x - 2, start.y));

        let mut game = Self {
            grid_width,
            grid_height,
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Position::new(0, 0),
            score: 0,
            game_over: false,
            tick_accumulator: 0.0,
            tick_rate: 0.3,
        };

        game.spawn_food();
        game
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if dir != self.direction.opposite() {
            self.next_direction = dir;
        }
    }

    fn spawn_food(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        loop {
            let pos = Position::new(
                rng.gen_range(0..self.grid_width),
                rng.gen_range(0..self.grid_height),
            );
            if !self.snake.iter().any(|s| s.x == pos.x && s.y == pos.y) {
                self.food = pos;
                break;
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.game_over {
            return;
        }

        self.tick_accumulator += delta_time;

        while self.tick_accumulator >= self.tick_rate {
            self.tick_accumulator -= self.tick_rate;
            self.tick();
        }
    }

    fn tick(&mut self) {
        self.direction = self.next_direction;

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position::new(head.x, head.y - 1),
            Direction::Down => Position::new(head.x, head.y + 1),
            Direction::Left => Position::new(head.x - 1, head.y),
            Direction::Right => Position::new(head.x + 1, head.y),
        };

        if new_head.x < 0
            || new_head.x >= self.grid_width
            || new_head.y < 0
            || new_head.y >= self.grid_height
        {
            self.game_over = true;
            return;
        }

        if self.snake.iter().any(|s| s.x == new_head.x && s.y == new_head.y) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 10;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> CellType {
        if self.snake.is_empty() {
            return CellType::Empty;
        }

        let head = self.snake.front().unwrap();
        let tail = self.snake.back().unwrap();

        if x == self.food.x && y == self.food.y {
            return CellType::Food;
        }

        if x == head.x && y == head.y {
            return CellType::Head;
        }

        if x == tail.x && y == tail.y {
            return CellType::Tail;
        }

        if self.snake.iter().any(|s| s.x == x && s.y == y) {
            return CellType::Body;
        }

        CellType::Empty
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.grid_width, self.grid_height);
    }
}
