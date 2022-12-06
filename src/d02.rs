
#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq, Clone, Copy)]
enum GameResult {
    Win,
    Loss,
    Draw
}

impl GameResult {
    fn new(name: &str) -> Self {
        match name {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            &_ => todo!()
        }
    }

    fn score(self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Draw => 3,
        }
    }
}

impl Move {
    fn new(name: &str) -> Self {
        match name {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            &_ => todo!()
        }
    }

    fn beats(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn loses(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn score(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

fn main() {
    let file_str = include_str!("../inputs/d02");

    let mut total_score = 0;
    for game in file_str.lines() {
        let mut split_game = game.split(" ");
        let opponnents_move = Move::new(split_game.nth(0).unwrap());
        let desired_outcome = GameResult::new(split_game.nth(0).unwrap());
        let my_move = pick_move(desired_outcome, opponnents_move);

        total_score += play_move(my_move, opponnents_move)
    }

    println!("total_score: {}", total_score)
}

fn pick_move(desired_outcome: GameResult, opponnents_move: Move) -> Move {
    if desired_outcome == GameResult::Loss {
        return opponnents_move.beats();
    }

    if desired_outcome == GameResult::Draw {
        return opponnents_move;
    }

    return opponnents_move.loses();
}

fn play_move(my_move: Move, opponnents_move: Move) -> i32 {
    let my_beats = my_move.beats();
    let opponnents_beats = opponnents_move.beats();
    
    if my_beats == opponnents_move {
        return GameResult::Win.score() + my_move.score();
    }
    if opponnents_beats == my_move {
        return GameResult::Loss.score() + my_move.score();
    }

    return GameResult::Draw.score() + my_move.score();
}