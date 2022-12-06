use util::get_input;

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from_str(s: &str) -> Option<Play> {
        match s {
            "A" => Some(Play::Rock),
            "B" => Some(Play::Paper),
            "C" => Some(Play::Scissors),
            "X" => Some(Play::Rock),
            "Y" => Some(Play::Paper),
            "Z" => Some(Play::Scissors),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn from_plays(p1: &Play, p2: &Play) -> Outcome {
        match (p2, p1) {
            (Play::Rock, Play::Paper) => Outcome::Lose,
            (Play::Rock, Play::Scissors) => Outcome::Win,
            (Play::Paper, Play::Rock) => Outcome::Win,
            (Play::Paper, Play::Scissors) => Outcome::Lose,
            (Play::Scissors, Play::Rock) => Outcome::Lose,
            (Play::Scissors, Play::Paper) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }
}

fn main() {
    get_input(|content| {
        let rows = content
            .trim()
            .split("\n")
            .map(|r| r.trim().split(" ").take(2).collect::<Vec<&str>>());
        let game1: Vec<usize> = rows.to_owned()
            .map(|row| {
                let p1 = Play::from_str(row[0]).unwrap();
                let p2 = Play::from_str(row[1]).unwrap();
                let outcome = Outcome::from_plays(&p1, &p2);
                let mut score = match p2 {
                    Play::Rock => 1,
                    Play::Paper => 2,
                    Play::Scissors => 3,
                };
                score = score + outcome.score();
                score
            })
            .collect();

        println!("game1: {:?}", game1.iter().sum::<usize>());

    let game2: Vec<usize> = rows.map(|row| {
            let their_play = Play::from_str(row[0]).unwrap();
            let outcome = match row[1] {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => Outcome::Draw,
            };

           let my_play =  match (their_play, outcome) {
                (_, Outcome::Draw) => their_play.clone(),

                (Play::Rock, Outcome::Lose) => Play::Scissors,
                (Play::Rock, Outcome::Win) => Play::Paper,
                (Play::Paper, Outcome::Lose) => Play::Rock,
                (Play::Paper, Outcome::Win) => Play::Scissors,
                (Play::Scissors, Outcome::Lose) => Play::Paper,
                (Play::Scissors, Outcome::Win) => Play::Rock,
            };
            let mut score = match my_play {
                Play::Rock => 1,
                Play::Paper => 2,
                Play::Scissors => 3,
            };

            score = score + outcome.score();
            score
            
            
        })
        .collect();

        println!("game2: {:?}", game2.iter().sum::<usize>());
    });
}
