fn main() {
    let mut score = 0;
    for ln in std::io::stdin().lines().map(|l| l.unwrap()) {
        let (opp, you) = parse_rps_line(&ln);
        score += you.shape_score() + you.vs(&opp).outcome_score();
    }
    println!("{}", score);
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Beats,
    Ties,
    Loses,
}

fn parse_rps_line(ln: &str) -> (RPS, RPS) {
    let pair: Vec<&str> = ln.split_whitespace().collect();
    let opponent = match pair[0] {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!(),
    };
    let outcome = match pair[1] {
        "X" => Outcome::Loses,
        "Y" => Outcome::Ties,
        "Z" => Outcome::Beats,
        _ => panic!(),
    };
    let you = rps_for_outcome(&outcome, &opponent);
    (opponent, you)
}

fn rps_for_outcome(outcome: &Outcome, vs: &RPS) -> RPS {
    let result = match (outcome, vs) {
        (Outcome::Beats, RPS::Scissors)
        | (Outcome::Ties, RPS::Rock)
        | (Outcome::Loses, RPS::Paper) => RPS::Rock,
        (Outcome::Beats, RPS::Rock)
        | (Outcome::Ties, RPS::Paper)
        | (Outcome::Loses, RPS::Scissors) => RPS::Paper,
        (Outcome::Beats, RPS::Paper)
        | (Outcome::Ties, RPS::Scissors)
        | (Outcome::Loses, RPS::Rock) => RPS::Scissors,
    };
    assert!(result.vs(vs) == *outcome);
    result
}

impl RPS {
    fn vs(&self, opp: &RPS) -> Outcome {
        match (self, opp) {
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => {
                Outcome::Beats
            }
            (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => {
                Outcome::Ties
            }
            _ => Outcome::Loses,
        }
    }

    fn shape_score(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl Outcome {
    fn outcome_score(self) -> i32 {
        match self {
            Outcome::Beats => 6,
            Outcome::Ties => 3,
            Outcome::Loses => 0,
        }
    }
}
