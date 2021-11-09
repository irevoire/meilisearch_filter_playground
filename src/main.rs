use std::io::{stdin, stdout, Write};
use termion::{
    color,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
    style,
};

fn main() {
    let _screen = AlternateScreen::from(stdout());

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut input = String::new();

    print!("{}", termion::cursor::Hide);

    draw_filter(&input);

    for event in stdin.events() {
        match event {
            Ok(Event::Key(Key::Char(c))) => input.push(c),
            Ok(Event::Key(Key::Backspace)) => drop(input.pop()),
            _ => break,
        }
        draw_filter(&input);
        stdout.flush().unwrap();
    }

    print!("{}", termion::cursor::Show);
}

fn draw_filter(filter: &str) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    match filter_parser::FilterCondition::parse(filter) {
        Ok(result) => {
            print!("🟢 {}\n\r", filter);
            print!("{:?}", result);
        }
        Err(error) => {
            print!("🔴 ");
            let error = error.to_string();
            let (diagnostic, position) = error.split_once('\n').unwrap();
            let (position, filter) = position.split_once(' ').unwrap();

            let (start, end) = position.split_once(':').unwrap();
            let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());
            let length = end - start;

            let mut chars = filter.chars();
            chars.by_ref().take(start - 1).for_each(|c| print!("{}", c));
            print!("{}{}", color::Fg(color::Red), style::Bold);
            if length == 0 {
                print!("{} ", color::Bg(color::Red));
            } else {
                chars.by_ref().take(length).for_each(|c| print!("{}", c));
            }
            print!("{}", style::Reset);
            chars.for_each(|c| print!("{}", c));
            print!("\r\n{}", diagnostic);
        }
    }
}
