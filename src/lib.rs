use std::str::Lines;

#[derive(Debug, Clone)]
pub struct ClawGames<'s> {
    lines: Lines<'s>,
}
impl<'s> ClawGames<'s> {
    pub fn new(s: &'s str) -> Self {
        Self { lines: s.lines() }
    }
}

impl<'s> Iterator for ClawGames<'s> {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        let a_line = loop {
            let next_line = self.lines.next()?;
            if !next_line.is_empty() {
                break next_line;
            }
        };
        let a_movement_substring = a_line.strip_prefix("Button A:").unwrap();
        let a_movement = parse_button_row(a_movement_substring, '+');
        let b_movement_substring = dbg!(self.lines.next().unwrap())
            .strip_prefix("Button B:")
            .unwrap();
        let b_movement = parse_button_row(b_movement_substring, '+');
        let prize_line = self.lines.next().unwrap().strip_prefix("Prize:").unwrap();
        let prize = parse_button_row(prize_line, '=');
        Some(Game {
            a_movement,
            b_movement,
            prize,
        })
    }
}

fn parse_button_row(s: &str, number_separator: char) -> (u32, u32) {
    let mut parts = s.trim().split(',');
    let x = parts
        .next()
        .unwrap()
        .trim()
        .strip_prefix(&format!("X{number_separator}"))
        .unwrap()
        .parse()
        .unwrap();
    let y = parts
        .next()
        .unwrap()
        .trim()
        .strip_prefix(&format!("Y{number_separator}"))
        .unwrap()
        .parse()
        .unwrap();
    (x, y)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Game {
    pub a_movement: (u32, u32),
    pub b_movement: (u32, u32),
    pub prize: (u32, u32),
}

#[cfg(test)]
mod test {
    use crate::{ClawGames, Game};

    #[test]
    fn single_example() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let mut claws = ClawGames::new(input);
        assert_eq!(
            claws.next(),
            Some(Game {
                a_movement: (94, 34),
                b_movement: (22, 67),
                prize: (8400, 5400)
            })
        );
        assert_eq!(claws.next(), None);
    }

    #[test]
    fn big_example() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let mut games = ClawGames::new(input);
        assert_eq!(
            games.next(),
            Some(Game {
                a_movement: (94, 34),
                b_movement: (22, 67),
                prize: (8400, 5400)
            })
        );
        assert_eq!(
            games.next(),
            Some(Game {
                a_movement: (26, 66),
                b_movement: (67, 21),
                prize: (12748, 12176)
            })
        );
        assert_eq!(
            games.next(),
            Some(Game {
                a_movement: (17, 86),
                b_movement: (84, 37),
                prize: (7870, 6450)
            })
        );
        assert_eq!(
            games.next(),
            Some(Game {
                a_movement: (69, 23),
                b_movement: (27, 71),
                prize: (18641, 10279)
            })
        );
        assert_eq!(games.next(), None);
    }
}
