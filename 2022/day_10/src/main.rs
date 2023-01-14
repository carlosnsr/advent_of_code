use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

enum Command {
    NoOp,
    AddX(isize),
}

#[derive(Debug)]
// Sprite: 3 pixels wide.  X is the position of the middle pixel
// A single pixel is drawn each cycle
struct Register {
    x: isize,
    cycles: usize,
    sum: isize,
    stream: Stream,
}

const DARK_PIXEL: char = '.';
const LIT_PIXEL: char = '#';
const SCREEN_X: usize = 40;
const SCREEN_Y: usize = 6;

type Stream = Vec<char>;
type Screen = Vec<Stream>;

impl Register {
    fn new() -> Self {
        Self { x: 1, cycles: 0, sum: 0, stream: Vec::with_capacity(SCREEN_X * SCREEN_Y) }
    }

    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, x: isize) {
        self.tick();
        self.tick();
        self.x += x;
    }

    fn tick(&mut self) {
        self.draw_pixel();
        self.cycles += 1;
        if self.is_signal() {
            self.sum += self.signal_strength();
        }
    }

    fn draw_pixel(&mut self) {
        let pixel_x = (self.cycles % SCREEN_X) as isize;
        let mut pixel = DARK_PIXEL;
        if self.x == pixel_x - 1 || self.x == pixel_x || self.x == pixel_x + 1 {
            pixel = LIT_PIXEL;
        }
        self.stream.push(pixel);
    }

    fn is_signal(&self) -> bool {
        match self.cycles {
            20 => true,
            60 => true,
            100 => true,
            140 => true,
            180 => true,
            220 => true,
            _ => false,
        }
    }

    fn signal_strength(&self) -> isize {
         self.x * (self.cycles as isize)
    }

    fn parse(line: &String) -> Command {
        let tokens: Vec<&str> = line.split(" ").collect();
        match tokens[0] {
            "noop" => Command::NoOp,
            "addx" => Command::AddX(tokens[1].parse::<isize>().unwrap()),
            _ => panic!(),
        }
    }

    fn print_screen(screen: &Screen) {
        for line in  screen.iter() {
            let string: String = line.iter().collect();
            println!("{:?}", string);
        }
        println!("");
    }

    fn make_screen(stream: &Stream) -> Screen {
        stream
            .chunks(SCREEN_X)
            .map(|chunk| {
                chunk
                    .iter()
                    .fold(Vec::with_capacity(SCREEN_X), |mut acc, c| {
                        acc.push(c.clone());
                        acc
                    })
            })
            .collect::<Screen>()
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut register = Register::new();
    for (_index, line) in reader.lines().enumerate() {
        match Register::parse(&line.unwrap()) {
            Command::NoOp => register.noop(),
            Command::AddX(x) => register.addx(x),
        }
    }
    println!("{}", register.sum);
    Register::print_screen(&Register::make_screen(&register.stream));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "addx 15",
            "addx -11",
            "addx 6",
            "addx -3",
            "addx 5",
            "addx -1",
            "addx -8",
            "addx 13",
            "addx 4",
            "noop",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx -35",
            "addx 1",
            "addx 24",
            "addx -19",
            "addx 1",
            "addx 16",
            "addx -11",
            "noop",
            "noop",
            "addx 21",
            "addx -15",
            "noop",
            "noop",
            "addx -3",
            "addx 9",
            "addx 1",
            "addx -3",
            "addx 8",
            "addx 1",
            "addx 5",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx -36",
            "noop",
            "addx 1",
            "addx 7",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "addx 6",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx 7",
            "addx 1",
            "noop",
            "addx -13",
            "addx 13",
            "addx 7",
            "noop",
            "addx 1",
            "addx -33",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "noop",
            "noop",
            "noop",
            "addx 8",
            "noop",
            "addx -1",
            "addx 2",
            "addx 1",
            "noop",
            "addx 17",
            "addx -9",
            "addx 1",
            "addx 1",
            "addx -3",
            "addx 11",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx -13",
            "addx -19",
            "addx 1",
            "addx 3",
            "addx 26",
            "addx -30",
            "addx 12",
            "addx -1",
            "addx 3",
            "addx 1",
            "noop",
            "noop",
            "noop",
            "addx -9",
            "addx 18",
            "addx 1",
            "addx 2",
            "noop",
            "noop",
            "addx 9",
            "noop",
            "noop",
            "noop",
            "addx -1",
            "addx 2",
            "addx -37",
            "addx 1",
            "addx 3",
            "noop",
            "addx 15",
            "addx -21",
            "addx 22",
            "addx -6",
            "addx 1",
            "noop",
            "addx 2",
            "addx 1",
            "noop",
            "addx -10",
            "noop",
            "noop",
            "addx 20",
            "addx 1",
            "addx 2",
            "addx 2",
            "addx -6",
            "addx -11",
            "noop",
            "noop",
            "noop",
        ]
    }

    #[test]
    fn test() {
        let mut register = Register::new();
        let mut last_sum = register.sum;
        for (index, line) in input().iter().enumerate() {
            let line: String = (*line).into();
            match Register::parse(&line) {
                Command::NoOp => register.noop(),
                Command::AddX(x) => register.addx(x),
            }
        }
        assert_eq!(register.sum, 13140);

        let expected_screen: Screen = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ].iter().map(|s| s.chars().collect() ).collect();
        let actual_screen: Screen = Register::make_screen(&register.stream);

        Register::print_screen(&actual_screen);
        Register::print_screen(&expected_screen);
        assert_eq!(actual_screen, expected_screen);
    }
}
