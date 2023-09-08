use crate::turns::Move;
use std::fmt::Display;

#[derive(Debug)]
pub struct Scramble {
    pub moves: Vec<Move>,
}

impl Scramble {
    pub fn new(length: usize) -> Self {
        let mut moves = vec![Move::random()];
        let mut pos = 0;
        while moves.len() < length {
            let mut next_move = Move::random();
            while next_move.direction == moves.last().unwrap().direction {
                next_move = Move::random();
            }

            if pos >= 2 {
                if moves.last().unwrap().direction.opposite() == next_move.direction {
                    while moves[moves.len() - 2].direction == next_move.direction
                        || next_move.direction == moves.last().unwrap().direction
                    {
                        next_move = Move::random();
                    }
                }
            }

            moves.push(next_move);
            pos += 1;
        }

        Scramble { moves }
    }
}

impl Display for Scramble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in 0..self.moves.len() {
            match write!(f, "{}{} ", self.moves[i].direction, self.moves[i].turn) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }
}
