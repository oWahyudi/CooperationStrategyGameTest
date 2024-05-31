pub trait Strategy {
    fn play_for_favoured_move(&mut self, favoured_move: Move) -> Move;
    fn handle_last_round(&mut self, round: Round, favoured_move: Move);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Move {
    X,
    Y,
    Z,
}

impl Move {
    pub fn opposite(self) -> Self {
        match self {
            Move::X => Move::Y,
            Move::Y => Move::X,
            Move::Z => Move::Z,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Round {
    pub my_move: Move,
    pub opponent_move: Move,
}

struct IdentifyAndExploit {
    round: u32,
}

impl IdentifyAndExploit {
    fn new() -> Self {
        Self { round: 0 }
    }
}

impl Strategy for IdentifyAndExploit {
    fn play_for_favoured_move(&mut self, favoured_move: Move) -> Move {
        self.round += 1;

        //4  44200
        //3  38100
        //2  32220
        if self.round % 5 == 0 || self.round % 5 == 2  {
			favoured_move
		} else {
			Move::Z
		}
    }

    fn handle_last_round(&mut self, _round: Round, _favoured_move: Move) {
     
    }
}

struct AlwaysX;

impl Strategy for AlwaysX {
    fn play_for_favoured_move(&mut self, _favoured_move: Move) -> Move {
        Move::Y
    }

    fn handle_last_round(&mut self, _round: Round, _favoured_move: Move) {}
}

struct Simulator {
    rounds: usize,
}

impl Simulator {
    fn new(rounds: usize) -> Self {
        Self { rounds }
    }

    fn run(&self, strategy1: &mut dyn Strategy, strategy2: &mut dyn Strategy) -> (usize, usize) {
        let mut score1 = 0;
        let mut score2 = 0;

        let player_1 = Move::Y;
        let player_2 = Move::X;

        for _ in 0..self.rounds {
            let move1 = strategy1.play_for_favoured_move(player_1.clone());
            dbg!(&move1);
            let move2 = strategy2.play_for_favoured_move(player_2.clone());
            dbg!(&move2);

            if player_1 == Move::X {
                let (score1_increment, score2_increment) = self.calculate_scores(move1, move2);
                dbg!(&score1_increment);
                dbg!(&score2_increment);
                score1 += score1_increment;
                score2 += score2_increment;
            } else {
                let (score2_increment, score1_increment) = self.calculate_scores(move2, move1);
                dbg!(&score1_increment);
                dbg!(&score2_increment);
                score1 += score1_increment;
                score2 += score2_increment;
            }

            let round = Round {
                my_move: move1,
                opponent_move: move2,
            };

            strategy1.handle_last_round(round.clone(), player_1.clone());
            strategy2.handle_last_round(round.clone(), player_2.clone());
        }

        (score1, score2)
    }

    fn calculate_scores(&self, move1: Move, move2: Move) -> (usize, usize) {
        match (move1, move2) {
            (Move::X, Move::X) => (250, 50),
            (Move::Y, Move::Y) => (50, 250),
            (Move::Z, Move::Z) => (100, 100),
            _ => (0, 0),
        }
    }
}

fn main() {
    let mut strategy1 = IdentifyAndExploit::new();
    let mut strategy2 = AlwaysX;

    let simulator = Simulator::new(101);
    let (score1, score2) = simulator.run(&mut strategy1, &mut strategy2);

    println!("Strategy 1 score: {}", score1);
    println!("Strategy 2 score: {}", score2);
}
