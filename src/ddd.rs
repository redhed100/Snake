use std::io::{self, Write};
// use std::thread;
use std::time::Duration;
use crossterm::{
    self, execute, queue,
    style, // ::{self, Stylize},
    cursor, terminal,
    event::{poll, read, Event, KeyEvent, KeyEventKind, KeyCode}
};

const SCREEN_WIDTH: u16 = 100;
const SCREEN_HEIGHT: u16 = 20;
const SCREEN_TITLE: &str = "ТЕТРИС";

// fn sleep(ms: u64) { thread::sleep(Duration::from_millis(ms)); }

// Экран ------------------------------------------------------------------------

fn start() -> io::Result<()> {
    execute!(io::stdout(),
        // cursor::Hide,
        terminal::SetSize(SCREEN_WIDTH, SCREEN_HEIGHT),
        terminal::SetTitle::<&str>(SCREEN_TITLE),
        cursor::MoveTo(0,0),
        terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}
fn print_frame() -> io::Result<()>{
    let mut out = io::stdout();

    queue!(out,
        cursor::MoveTo(0,0),
        style::SetForegroundColor(style::Color::Magenta))?;

    for y in 0..SCREEN_HEIGHT {
        queue!(out, cursor::MoveTo(0,y))?;
        if y == 0 || y == SCREEN_HEIGHT - 1 {
            for _ in 0..SCREEN_WIDTH {
                queue!(out, style::Print("█"))?;
            }
        } else {
            queue!(out, style::Print("██"))?;
            for _ in 2..SCREEN_WIDTH - 2 {
                queue!(out, style::Print(" "))?;
            }
            queue!(out, style::Print("██"))?;
        }
    }

    queue!(out,
        cursor::MoveTo(SCREEN_WIDTH/2 - 10, SCREEN_HEIGHT/2 - 1),
        style::SetForegroundColor(style::Color::White),
        style::Print("Выход: Esc или Q"),
        cursor::MoveTo(0,0),
        style::SetForegroundColor(style::Color::Green),
    )?;

    out.flush()?;
    Ok(())
}
fn finish() {
    execute!(io::stdout(),
        cursor::Show,
        cursor::MoveTo(0,0),
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All)).unwrap();
    std::process::exit(0);
}

// Клавиатура --------------------------------------------------------------------

fn handle_key_event(event: KeyEvent) {
    let mut out = io::stdout();
    if event.kind == KeyEventKind::Press {
        match event.code {
            KeyCode::Esc => finish(),
            KeyCode::Enter => print_frame().unwrap(),
            KeyCode::Char('q' | 'Q') => { finish() },
            KeyCode::Char(ch) =>
                { execute!(out, style::Print(ch)).unwrap() },
            KeyCode::Down =>  execute!(out, cursor::MoveDown(1)).unwrap(),
            KeyCode::Up =>    execute!(out, cursor::MoveUp(1)).unwrap(),
            KeyCode::Left =>  execute!(out, cursor::MoveLeft(1)).unwrap(),
            KeyCode::Right => execute!(out, cursor::MoveRight(1)).unwrap(),
            _ => println!("Pressed: {:?}",event.code)
        }
    }
}

fn handle_events(wait_ms: u64) {
    loop {
        if poll(Duration::from_millis(wait_ms)).unwrap_or(false) {
            let res = read();
            if res.is_ok() {
                match res.unwrap() {
                    Event::Key(event) => handle_key_event(event),
                    _ => {}
                }
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
}

fn main() -> io::Result<()> {
    start()?;
    print_frame()?;
    handle_events(500);
    Ok(())
}