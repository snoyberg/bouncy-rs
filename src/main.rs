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

impl std::fmt::Display for Game {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut std::fmt::Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            write!(fmt, "+\n")
        };

        top_bottom(fmt)?;

        for y in 0..self.frame.height {
            write!(fmt, "|")?;
            for x in 0..self.frame.width {
                let c =
                       if x == self.ball.x && y == self.ball.y {
                           'o'
                       } else {
                           ' '
                       };
                write!(fmt, "{}", c)?;
            }
            write!(fmt, "|\n")?;
        }

        top_bottom(fmt)
    }
}

fn main() {
    let mut game = Game {
        frame: Frame {
            width: 80,
            height: 30,
        },
        ball: Ball {
            x: 1,
            y: 2,
            horiz: Horiz::Left,
            vert: Vert::Up,
        }
    };

    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
