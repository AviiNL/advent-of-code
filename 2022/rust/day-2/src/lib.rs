#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RockPaperScissors {
    Rock(i32),
    Paper(i32),
    Scissors(i32),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Game {
    player1: RockPaperScissors,
    player2: RockPaperScissors,
    player1_points: i32,
    player2_points: i32,
}

// Part-1
impl Game {
    fn from_part_1(input: &str) -> Self {
        // Input looks like A Y, or B X, or C Z
        // Where A and X are Rock
        // Where B and Y are Paper
        // Where C and Z are Scissors
        let mut input = input.split_whitespace();
        let player1 = match input.next().unwrap() {
            "A" => RockPaperScissors::Rock(1),
            "B" => RockPaperScissors::Paper(2),
            "C" => RockPaperScissors::Scissors(3),
            _ => panic!("Invalid input"),
        };

        let player2 = match input.next().unwrap() {
            "X" => RockPaperScissors::Rock(1),
            "Y" => RockPaperScissors::Paper(2),
            "Z" => RockPaperScissors::Scissors(3),
            _ => panic!("Invalid input"),
        };

        let (p1p, p2p) = match (player1, player2) {
            (RockPaperScissors::Rock(p1), RockPaperScissors::Rock(p2)) => (p1 + 3, p2 + 3),
            (RockPaperScissors::Rock(p1), RockPaperScissors::Paper(p2)) => (p1, p2 + 6),
            (RockPaperScissors::Rock(p1), RockPaperScissors::Scissors(p2)) => (p1 + 6, p2),

            (RockPaperScissors::Paper(p1), RockPaperScissors::Rock(p2)) => (p1 + 6, p2),
            (RockPaperScissors::Paper(p1), RockPaperScissors::Paper(p2)) => (p1 + 3, p2 + 3),
            (RockPaperScissors::Paper(p1), RockPaperScissors::Scissors(p2)) => (p1, p2 + 6),

            (RockPaperScissors::Scissors(p1), RockPaperScissors::Rock(p2)) => (p1, p2 + 6),
            (RockPaperScissors::Scissors(p1), RockPaperScissors::Paper(p2)) => (p1 + 6, p2),
            (RockPaperScissors::Scissors(p1), RockPaperScissors::Scissors(p2)) => (p1 + 3, p2 + 3),
        };

        Self {
            player1,
            player2,
            player1_points: p1p,
            player2_points: p2p,
        }
    }
}

// Part-2
impl Game {
    fn from_part_2(input: &str) -> Self {
        // Input looks like A Y, or B X, or C Z
        // Where A and X are Rock
        // Where B and Y are Paper
        // Where C and Z are Scissors
        let mut input = input.split_whitespace();
        let player1 = match input.next().unwrap() {
            "A" => RockPaperScissors::Rock(1),
            "B" => RockPaperScissors::Paper(2),
            "C" => RockPaperScissors::Scissors(3),
            _ => panic!("Invalid input"),
        };

        let player2 = match input.next().unwrap() {
            "X" =>
            // X means we need to lose
            {
                match player1 {
                    RockPaperScissors::Rock(_) => RockPaperScissors::Scissors(3),
                    RockPaperScissors::Paper(_) => RockPaperScissors::Rock(1),
                    RockPaperScissors::Scissors(_) => RockPaperScissors::Paper(2),
                }
            }

            "Y" =>
            // Y means we need to draw
            {
                match player1 {
                    RockPaperScissors::Rock(_) => RockPaperScissors::Rock(1),
                    RockPaperScissors::Paper(_) => RockPaperScissors::Paper(2),
                    RockPaperScissors::Scissors(_) => RockPaperScissors::Scissors(3),
                }
            }
            "Z" =>
            // Z means we need to win
            {
                match player1 {
                    RockPaperScissors::Rock(_) => RockPaperScissors::Paper(2),
                    RockPaperScissors::Paper(_) => RockPaperScissors::Scissors(3),
                    RockPaperScissors::Scissors(_) => RockPaperScissors::Rock(1),
                }
            }
            _ => panic!("Invalid input"),
        };

        let (p1p, p2p) = match (player1, player2) {
            (RockPaperScissors::Rock(p1), RockPaperScissors::Rock(p2)) => (p1 + 3, p2 + 3),
            (RockPaperScissors::Rock(p1), RockPaperScissors::Paper(p2)) => (p1, p2 + 6),
            (RockPaperScissors::Rock(p1), RockPaperScissors::Scissors(p2)) => (p1 + 6, p2),

            (RockPaperScissors::Paper(p1), RockPaperScissors::Rock(p2)) => (p1 + 6, p2),
            (RockPaperScissors::Paper(p1), RockPaperScissors::Paper(p2)) => (p1 + 3, p2 + 3),
            (RockPaperScissors::Paper(p1), RockPaperScissors::Scissors(p2)) => (p1, p2 + 6),

            (RockPaperScissors::Scissors(p1), RockPaperScissors::Rock(p2)) => (p1, p2 + 6),
            (RockPaperScissors::Scissors(p1), RockPaperScissors::Paper(p2)) => (p1 + 6, p2),
            (RockPaperScissors::Scissors(p1), RockPaperScissors::Scissors(p2)) => (p1 + 3, p2 + 3),
        };

        Self {
            player1,
            player2,
            player1_points: p1p,
            player2_points: p2p,
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let games = input.lines().map(Game::from_part_1).collect::<Vec<_>>();

    // sum the score of player2
    let score = games.iter().fold(0, |acc, game| acc + game.player2_points);

    score.to_string()
}

pub fn process_part2(input: &str) -> String {
    let games = input.lines().map(Game::from_part_2).collect::<Vec<_>>();

    // sum the score of player2
    let score = games.iter().fold(0, |acc, game| acc + game.player2_points);

    score.to_string()
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    pub fn test_part_1() {
        assert_eq!(super::process_part1(INPUT), "15");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(super::process_part2(INPUT), "12");
    }
}
