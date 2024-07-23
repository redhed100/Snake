use std::collections::VecDeque;
use std::io::stdout;
use std::time::{Duration, Instant};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{cursor, execute, style};
use crossterm::style::{style, Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::ClearType;



#[derive(Copy, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
    //None
}

#[derive(Copy, Clone)]
enum Var {
    Lr,
    Ud,
    Lu,
    Ld,
    Ur,
    Dr
}

// impl Dir {
//     pub fn clone(&self) -> Dir {
//         match self {
//             Dir::Up => Dir::Up,
//             Dir::Left => Dir::Left,
//             Dir::Down => Dir::Down,
//             Dir::Right => Dir::Right,
//             Dir::None => Dir::None
//         }
//     }
// }

#[derive(Copy, Clone)]
struct Pos {
    x: u16,
    y: u16
}

impl Pos {
    pub fn new(x: u16, y: u16) -> Pos {
        Pos { x, y }
    }
}

struct Game {
    snake: Snake,
    map: Map,
    speed: u16,
    time: f32
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        // Game { snake: Snake::new((width+1)/2,(height+1)/2), map: Map::new(width, height), speed: 1, time: 0.0}
        Game { snake: Snake::new(3, 1), map: Map::new(width, height), speed: 1, time: 0.0}
    }

    pub fn is_step(&self) -> bool {
        if self.time >= 1.0 { true }
        else { false }
    }

    pub fn sync(&mut self, fElapsedTime: &f32) {
        self.time += self.speed as f32 * fElapsedTime;
    }

    pub fn step(&mut self) {
        self.snake.step(&self.map);
        self.time = 0.0;
    }
}

struct Snake {
    snake: VecDeque<Pos>,
    dir: Dir
    // headX: u16,
    // headY: u16
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        let mut snake = Snake { snake: VecDeque::new(), dir: Dir::Right};
        snake.snake.push_back(Pos::new(x-2, y));
        snake.snake.push_back(Pos::new(x-1, y));
        snake.snake.push_back(Pos::new(x, y));

        snake
    }

    fn head_x(&self) -> u16 {
        self.snake.back().unwrap().x
    }

    fn head_y(&self) -> u16 {
        self.snake.back().unwrap().y
    }

    pub fn get_dir(&self) -> Dir {
        self.dir //.clone()
    }

    pub fn get_var(&self, i: usize) -> Var {
        if self.sget(i-1).x+1 == self.sget(i).x && self.sget(i+1).x-1 == self.sget(i).x ||
           self.sget(i-1).x-1 == self.sget(i).x && self.sget(i+1).x+1 == self.sget(i).x {
            Var::Lr // Left - Right
        }
        else if self.sget(i-1).y+1 == self.sget(i).y && self.sget(i+1).y-1 == self.sget(i).y ||
                self.sget(i-1).y-1 == self.sget(i).y && self.sget(i+1).y+1 == self.sget(i).y {
            Var::Ud // Up - Down
        }
        else if self.sget(i-1).x+1 == self.sget(i).x && self.sget(i+1).y+1 == self.sget(i).y ||
                self.sget(i-1).y+1 == self.sget(i).y && self.sget(i+1).x+1 == self.sget(i).x  {
            Var::Lu // Left - Up
        }
        else if self.sget(i-1).x+1 == self.sget(i).x && self.sget(i+1).y-1 == self.sget(i).y ||
                self.sget(i-1).y-1 == self.sget(i).y && self.sget(i+1).x+1 == self.sget(i).x {
            Var::Ld // Left - Down
        }
        else if self.sget(i-1).y+1 == self.sget(i).y && self.sget(i+1).x-1 == self.sget(i).x ||
                self.sget(i-1).x-1 == self.sget(i).x && self.sget(i+1).y+1 == self.sget(i).y {
            Var::Ur // Up - Right
        }
        else {
            Var::Dr // Down - Right
        }
    }

    pub fn sget(&self, i: usize) -> Pos {
        *self.snake.get(i).unwrap()
    }

    pub fn is_dead(&self, map: &Map) -> bool {
        if self.head_x() <= 0 || self.head_x() > map.width || self.head_y() <= 0 || self.head_y() > map.height { return true; }
        return false
    }

    // pub fn move_head(&mut self) {
    //     match self.dir {
    //         Dir::Up => { self.headY() -= 1 },
    //         Dir::Left => { self.headX() -= 1 },
    //         Dir::Down => { self.headY() += 1 },
    //         Dir::Right => { self.head() += 1 },
    //         _ => {}
    //     }
    // }

    pub fn move_full(&mut self) -> Pos { // выдает Pos бывшего хвоста
        let mut p = self.snake.back().unwrap().clone();
        match self.dir {
            Dir::Up => { p.y -= 1 },
            Dir::Left => { p.x -= 1 },
            Dir::Down => { p.y += 1 },
            Dir::Right => { p.x += 1 },
        }
        self.snake.push_back(p);
        self.snake.pop_front().unwrap()
    }

    pub fn step(&mut self, map: &Map) {
        let last_pix =  self.move_full();
        let is_dead = self.is_dead(&map);
        // println!(" last_pix = {}, {} ", last_pix.x, last_pix.y);
        self.display(last_pix, is_dead);
    }

    pub fn display_full(&self) {
        execute!(stdout(), SetForegroundColor(Color::Rgb { r: 66, g: 111, b: 227 })).unwrap();
        let mut last_pix = Pos::new(0, 0);
        for (i, pSnakePos) in self.snake.iter().enumerate() {
            if (pSnakePos.x+pSnakePos.y) % 2 == 0 { execute!(stdout(), SetBackgroundColor(Color::Rgb { r: 170, g: 215, b: 81 })).unwrap(); }
            else { execute!(stdout(), SetBackgroundColor(Color::Rgb { r: 162, g: 209, b: 73 })).unwrap(); }
            if i == 0 {
                if self.sget(1).y+1 == self.sget(0).y {
                    execute!(stdout(),
                        cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                        style::Print("\\/")
                    ).unwrap();
                }
                if self.sget(1).x+1 == self.sget(0).x {
                    execute!(stdout(),
                        cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                        style::Print("═>")
                    ).unwrap();
                }
                if self.sget(1).y-1 == self.sget(0).y {
                    execute!(stdout(),
                        cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                        style::Print("/\\")
                    ).unwrap();
                }
                if self.sget(1).x-1 == self.sget(0).x {
                    execute!(stdout(),
                        cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                        style::Print("<═")
                    ).unwrap();
                }
            }
            else if i > 0 && i < self.snake.len() - 1 {
                match self.get_var(i) {
                    Var::Lr => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("══")
                        ).unwrap();
                    },
                    Var::Ud => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("││")
                        ).unwrap();
                    },
                    Var::Lu => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("╧┘")
                        ).unwrap();
                    },
                    Var::Ld => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("╤┐")
                    ).unwrap();
                    },
                    Var::Ur => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("└╧")
                        ).unwrap();
                    },
                    Var::Dr => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("┌╤")
                        ).unwrap();
                    }
                }
            }
            else /* if i >= self.snake.len() - 1 */ {
                match self.dir {
                    Dir::Up => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("/\\")
                        ).unwrap();
                    },
                    Dir::Left => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("<═")
                        ).unwrap();
                    },
                    Dir::Down => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("\\/")
                        ).unwrap();
                    },
                    Dir::Right => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("═>")
                        ).unwrap();
                    },
                    //_ => {}
                }
            }


            last_pix = pSnakePos.clone();
        }
        execute!(stdout(), ResetColor).unwrap();
        /*

    /\
┌╤══╧┘  ┌╤══╤┐
││  ┌╤══╧┘<═╧┘
└╧══╧┘
tail:    Rgb { r: 66, g: 111, b: 227 }
head:    Rgb { r: 78, g: 124, b: 246 }

lightBG: Rgb { r: 170, g: 215, b: 81 }
darkBG:  Rgb { r: 162, g: 209, b: 73 }


        ││
        ▕▏
╒╤──══──╧╛      ││
▕▏              ▕▏
││      ╒╤──══──╧╛
▕▏      ▕▏
╘╧──══──╧╛

        */
    }

    pub fn display(&self, last_pix: Pos, is_dead: bool) {
        execute!(stdout(), SetForegroundColor(Color::Rgb { r: 66, g: 111, b: 227 })).unwrap();
        if (last_pix.x+last_pix.y) % 2 == 0 { execute!(stdout(), SetBackgroundColor(Color::Rgb { r: 170, g: 215, b: 81 })).unwrap(); }
        else { execute!(stdout(), SetBackgroundColor(Color::Rgb { r: 162, g: 209, b: 73 })).unwrap(); }
        execute!(stdout(),
            cursor::MoveTo(last_pix.x*2+1, last_pix.y+1),
            style::Print("  ")
        ).unwrap();

        if self.sget(1).y+1 == self.sget(0).y {
            execute!(stdout(),
                cursor::MoveTo(self.sget(0).x*2+1,self.sget(0).y+1),
                style::Print("\\/")
            ).unwrap();
        }
        else if self.sget(1).x+1 == self.sget(0).x {
            execute!(stdout(),
                cursor::MoveTo(self.sget(0).x*2+1,self.sget(0).y+1),
                style::Print("═>")
            ).unwrap();
        }
        else if self.sget(1).y-1 == self.sget(0).y {
            execute!(stdout(),
                cursor::MoveTo(self.sget(0).x*2+1,self.sget(0).y+1),
                style::Print("/\\")
            ).unwrap();
        }
        else if self.sget(1).x-1 == self.sget(0).x {
            execute!(stdout(),
                cursor::MoveTo(self.sget(0).x*2+1,self.sget(0).y+1),
                style::Print("<═")
            ).unwrap();
        }

        {
            let i = self.snake.len()-2;
            let p_snake_pos = self.sget(i);
            match self.get_var(i) {
                Var::Lr => {
                    execute!(stdout(),
                        cursor::MoveTo(p_snake_pos.x*2+1,p_snake_pos.y+1),
                        style::Print("══")
                    ).unwrap();
                },
                Var::Ud => {
                    execute!(stdout(),
                        cursor::MoveTo(p_snake_pos.x*2+1,p_snake_pos.y+1),
                        style::Print("││")
                    ).unwrap();
                },
                Var::Lu => {
                    execute!(stdout(),
                        cursor::MoveTo(p_snake_pos.x*2+1,p_snake_pos.y+1),
                        style::Print("╧┘")
                    ).unwrap();
                },
                Var::Ld => {
                    execute!(stdout(),
                        cursor::MoveTo(p_snake_pos.x*2+1,p_snake_pos.y+1),
                        style::Print("╤┐")
                ).unwrap();
                },
                Var::Ur => {
                    execute!(stdout(),
                        cursor::MoveTo(p_snake_pos.x*2+1,p_snake_pos.y+1),
                        style::Print("└╧")
                    ).unwrap();
                },
                Var::Dr => {
                    execute!(stdout(),
                        cursor::MoveTo(p_snake_pos.x*2+1,p_snake_pos.y+1),
                        style::Print("┌╤")
                    ).unwrap();
                }
            }
        } 

        if !is_dead {
            execute!(stdout(),
                cursor::MoveTo(self.sget(self.snake.len()-1).x*2+1,self.sget(self.snake.len()-1).y+1),
                style::Print("##")
            ).unwrap();
        } else {

        }
        execute!(stdout(), ResetColor).unwrap();
    }
}

struct Map {
    width: u16,
    height: u16
}

impl Map {
    pub fn new(width: u16, height: u16) -> Map {
        Map { width, height }
    }

    pub fn display(&self) {            //   ┌ ─ ┐ │ └ ─ ┘
        execute!(stdout(),
            cursor::MoveTo(0, 0),
            style::SetBackgroundColor(Color::Rgb {r: 60, g: 60, b: 60}),
            style::SetForegroundColor(Color::Rgb {r: 200, g: 200, b: 200}),
        ).unwrap();
        for y in 0..self.height+2 {
            if y == 0 {
                execute!(stdout(), style::Print("┌")).unwrap();
                for _x in 0..self.width*2 { execute!(stdout(), style::Print("─")).unwrap(); }
                execute!(stdout(), style::Print("┐")).unwrap();
            }
            else if y < self.height+1 {
                execute!(stdout(), cursor::MoveTo(0, y), style::Print("│")).unwrap();
                execute!(stdout(), cursor::MoveTo(self.width*2+1, y), style::Print("│")).unwrap();
            }
            else if y == self.height+1 {
                execute!(stdout(), cursor::MoveTo(0, y) , style::Print("└")).unwrap();
                for _x in 0..self.width*2 { execute!(stdout(), style::Print("─")).unwrap(); }
                execute!(stdout(), style::Print("┘")).unwrap();
            }
        }

        execute!(stdout(), cursor::MoveTo(1, 1)).unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                if (y+x) % 2 == 0 {         // Light
                    execute!(stdout(),
                        cursor::MoveTo(x*2+1, y+1),
                        style::SetBackgroundColor(Color::Rgb { r: 170, g: 215, b: 81 }),
                        style::Print("  ")
                    ).unwrap();
                }
                else {                      // Dark
                    execute!(stdout(),
                        cursor::MoveTo(x*2+1, y+1),
                        style::SetBackgroundColor(Color::Rgb { r: 162, g: 209, b: 73 }),
                        style::Print("  ")
                    ).unwrap();
                }
            }
            execute!(stdout(), cursor::MoveTo(1, y+1)).unwrap();
        }
        execute!(stdout(), ResetColor).unwrap();
    }
}

fn start() {
    execute!(stdout(),
        crossterm::cursor::Hide,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::terminal::Clear(ClearType::All)
    ).unwrap();
}

fn finish() {
    execute!(stdout(),
        crossterm::terminal::SetSize(WIDTH, HEIGHT),
        crossterm::cursor::Show,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::terminal::Clear(ClearType::All)
    ).unwrap();

    std::process::exit(0);
}

fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}

static WIDTH: u16 = 120;
static HEIGHT: u16 = 30;

fn main() {
    start();
    let mut AppRun = true;
    let mut out = stdout();

    let mut game = Game::new(20, 20);
    game.map.display();
    game.snake.display_full();

    let mut s1 = Instant::now();
    while AppRun {
        let elapsedTime = s1.elapsed();
        let fElapsedTime: f32 = elapsedTime.as_micros() as f32/1_000_000.0;
        s1 = Instant::now();

        if poll(Duration::from_millis(0)).unwrap_or(false) {
            let res = read();
            if res.is_ok() {
                match res.unwrap() {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Esc => finish(),
                            KeyCode::Char('w') => game.snake.dir = Dir::Up,
                            KeyCode::Char('a') => game.snake.dir = Dir::Left,
                            KeyCode::Char('s') => game.snake.dir = Dir::Down,
                            KeyCode::Char('d') => game.snake.dir = Dir::Right,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        if game.is_step() {
            game.step();
        }

        game.sync(&fElapsedTime);

        let FPS = 1.0/fElapsedTime;
        let s = "FPS: ".to_string() + FPS.to_string().as_str();
        execute!(stdout(), crossterm::terminal::SetTitle(s)).unwrap();
    }
}
