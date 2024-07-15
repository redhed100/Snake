use std::collections::VecDeque;
use std::io::stdout;
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{cursor, execute, style};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::ClearType;



#[derive(Copy, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
    None
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
    map: Map
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game { snake: Snake::new((width+1)/2,(height+1)/2), map: Map::new(width, height) }
    }

    pub fn step(&mut self) {
        self.snake.step(&self.map);
    }
}

struct Snake {
    snake: VecDeque<(Pos, Dir)>,
    dir: Dir
    // headX: u16,
    // headY: u16
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Snake {
        let mut snake = Snake { snake: VecDeque::new(), dir: Dir::Right};
        snake.snake.push_back((Pos::new(x-2, y), Dir::Right));
        snake.snake.push_back((Pos::new(x-1, y), Dir::Right));
        snake.snake.push_back((Pos::new(x, y), Dir::Right));

        snake
    }

    fn headX(&self) -> u16 {
        self.snake.back().unwrap().0.x
    }

    fn headY(&self) -> u16 {
        self.snake.back().unwrap().0.y
    }

    pub fn getDir(&self) -> Dir {
        self.dir //.clone()
    }

    pub fn is_dead(&self, map: &Map) -> bool {
        if self.headX() <= 0 || self.headX() > map.width || self.headY() <= 0 || self.headY() > map.height { return true; }
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

    pub fn move_full(&mut self) -> (Pos, Dir) { // выдает Pos и Dir бывшего хвоста
        let mut p = self.snake.back().unwrap().0;
        match self.dir {
            Dir::Up => { p.y -= 1 },
            Dir::Left => { p.x -= 1 },
            Dir::Down => { p.y += 1 },
            Dir::Right => { p.x += 1 },
            _ => {}
        }
        // self.move_head();
        self.snake.push_back((p, self.dir.clone()));
        self.snake.pop_front().unwrap()
    }

    pub fn step(&mut self, map: &Map) {
        let last_pix =  self.move_full();
        let isDead = self.is_dead(&map);
        self.display(last_pix, isDead);
    }

    pub fn display_full(&self) {
        execute!(stdout(), SetForegroundColor(Color::Rgb { r: 66, g: 111, b: 227 }));
        let mut lastPix = Pos::new(0, 0);
        for (i, (pSnakePos, pSnakeDir)) in self.snake.iter().enumerate() {
            if (pSnakePos.x+pSnakePos.y) % 2 == 0 { execute!(stdout(), SetBackgroundColor(Color::Rgb { r: 170, g: 215, b: 81 })); }
            else { execute!(stdout(), SetBackgroundColor(Color::Rgb { r: 162, g: 209, b: 73 })); }
            if i == 0 {
                match pSnakeDir {
                    Dir::Up => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("\\/")
                        );
                    },
                    Dir::Left => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("═>")
                        );
                    },
                    Dir::Down => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("/\\")
                        );
                    },
                    Dir::Right => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("<═")
                        );
                    },
                    _ => {}
                }
            }
            else if i > 0 && i < self.snake.len() - 1 {
                match pSnakeDir {
                    Dir::Up => {
                        if lastPix.x == pSnakePos.x - 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("╧┘"));
                        }
                        else if lastPix.x == pSnakePos.x + 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("└╧"));
                        }
                        else if lastPix.y == pSnakePos.y + 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("││"));
                        }
                    },
                    Dir::Left => {
                        if lastPix.y == pSnakePos.y - 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("╧┘"));
                        }
                        else if lastPix.x == pSnakePos.x + 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("══"));
                        }
                        else if lastPix.y == pSnakePos.y + 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("╤┐"));
                        }
                    },
                    Dir::Down => {
                        if lastPix.x == pSnakePos.x - 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("╤┐"));
                        }
                        else if lastPix.y == pSnakePos.y - 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("││"));
                        }
                        else if lastPix.x == pSnakePos.x + 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("┌╤"));
                        }
                    },
                    Dir::Right => {
                        if lastPix.x == pSnakePos.x - 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("══"));
                        }
                        else if lastPix.y == pSnakePos.y - 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("└╧"));
                        }
                        else if lastPix.y == pSnakePos.y + 1 {
                            execute!(stdout(), cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1), style::Print("┌╤"));
                        }
                    },
                    _ => {}
                }
            }
            else /* if i >= self.snake.len() - 1 */ {
                match pSnakeDir {
                    Dir::Up => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("/\\")
                        );
                    },
                    Dir::Left => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("<═")
                        );
                    },
                    Dir::Down => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("\\/")
                        );
                    },
                    Dir::Right => {
                        execute!(stdout(),
                            cursor::MoveTo(pSnakePos.x*2+1,pSnakePos.y+1),
                            style::Print("═>")
                        );
                    },
                    _ => {}
                }
            }


            lastPix = pSnakePos.clone();
        }
        execute!(stdout(), ResetColor);
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

    pub fn display(&self, last_pix: (Pos, Dir), isDead: bool) {

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
        );
        for y in 0..self.height+2 {
            if y == 0 {
                execute!(stdout(), style::Print("┌"));
                for x in 0..self.width*2 { execute!(stdout(), style::Print("─")); }
                execute!(stdout(), style::Print("┐"));
            }
            else if y < self.height+1 {
                execute!(stdout(), cursor::MoveTo(0, y), style::Print("│"));
                execute!(stdout(), cursor::MoveTo(self.width*2+1, y), style::Print("│"));
            }
            else if y == self.height+1 {
                execute!(stdout(), cursor::MoveTo(0, y) , style::Print("└"));
                for x in 0..self.width*2 { execute!(stdout(), style::Print("─")); }
                execute!(stdout(), style::Print("┘"));
            }
        }

        execute!(stdout(), cursor::MoveTo(1, 1));
        for y in 0..self.height {
            for x in 0..self.width {
                if (y+x) % 2 == 0 {         // Light
                    execute!(stdout(),
                        cursor::MoveTo(x*2+1, y+1),
                        style::SetBackgroundColor(Color::Rgb { r: 170, g: 215, b: 81 }),
                        style::Print("  ")
                    );
                }
                else {                      // Dark
                    execute!(stdout(),
                        cursor::MoveTo(x*2+1, y+1),
                        style::SetBackgroundColor(Color::Rgb { r: 162, g: 209, b: 73 }),
                        style::Print("  ")
                    );
                }
            }
            execute!(stdout(), cursor::MoveTo(1, y+1));
        }
        execute!(stdout(), ResetColor);
    }
}

fn start() {
    execute!(stdout(),
        crossterm::cursor::Hide,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::terminal::Clear(ClearType::All)
    );
}

fn finish() {
    execute!(stdout(),
        crossterm::terminal::SetSize(WIDTH, HEIGHT),
        crossterm::cursor::Show,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::terminal::Clear(ClearType::All)
    );

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
        let mut elapsedTime = s1.elapsed();
        let mut fElapsedTime: f32 = elapsedTime.as_micros() as f32/1_000_000.0;
        s1 = Instant::now();

        if poll(Duration::from_millis(500)).unwrap_or(false) {
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

        game.step();

        let FPS = 1.0/fElapsedTime;
        let s = "FPS: ".to_string() + FPS.to_string().as_str();
        execute!(stdout(), crossterm::terminal::SetTitle(s));
    }
}
