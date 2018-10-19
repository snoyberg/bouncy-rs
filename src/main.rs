extern crate pancurses;

use pancurses::{initscr, endwin, Input};

enum Vert {
    Up,
    Down,
}

enum Horiz {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert: Vert,
    horiz: Horiz,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Ball {
    fn step(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz = Horiz::Right;
        } else if self.x == frame.width - 1 {
            self.horiz = Horiz::Left;
        }

        if self.y == 0 {
            self.vert = Vert::Down;
        } else if self.y == frame.height - 1 {
            self.vert = Vert::Up;
        }

        match self.horiz {
            Horiz::Left => self.x -= 1,
            Horiz::Right => self.x += 1,
        }
        match self.vert{
            Vert::Up => self.y -= 1,
            Vert::Down => self.y += 1,
        }
    }
}

impl Game {
    fn step(&mut self) {
        self.ball.step(&self.frame);
    }
}

fn main() {
    let window = initscr();
    window.timeout(30);

    let new_game = || {
        Game {
            frame: Frame {
                width: window.get_max_x() as u32 - 2,
                height: window.get_max_y() as u32 - 2,
            },
            ball: Ball {
                x: 1,
                y: 2,
                horiz: Horiz::Left,
                vert: Vert::Up,
            }
        }
    };
    let mut game = new_game();

    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            Some(Input::KeyResize) => {
                pancurses::resize_term(0, 0);
                game = new_game();
            },
            _ => (),
        }

        window.clear();
        window.border(
            '║',
            '║',
            '═',
            '═',
            '╔',
            '╗',
            '╚',
            '╝'
        );
        window.mvaddch(game.ball.y as i32 + 1, game.ball.x as i32 + 1, 'o');
        window.refresh();
        window.mv(0, 0);
        game.step();
    }

    endwin();
}
