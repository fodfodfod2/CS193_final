use super::*;

/* This represents possible game states */
#[derive(PartialEq)]
pub enum State {
    BlindSelect,
    Tutorial,
    SelectingHand,
    PlayTarot,
    HandPlayed,
    RoundEval,
    DrawToHand,
    Shop,
    MainMenu,
    GameOver,
    NewRound,
    Splash,
    RedeemCustomVoucher,
    PackOpening,
    Sandbox,
}

impl State {
    pub fn from(int: i32) -> Self {
        match int {
            1 => State::SelectingHand,
            2 => State::HandPlayed,
            3 => State::DrawToHand,
            4 => State::GameOver,
            5 => State::Shop,
            6 => State::PlayTarot,
            7 => State::BlindSelect,
            8 => State::RoundEval,
            9 => State::PackOpening,
            10 => State::PackOpening,
            11 => State::MainMenu,
            12 => State::Tutorial,
            13 => State::Splash,
            14 => State::Sandbox,
            15 => State::PackOpening,
            17 => State::PackOpening,
            18 => State::PackOpening,
            19 => State::NewRound,
            998 => State::RedeemCustomVoucher,
            999 => State::PackOpening,
            n => panic!("Unrecognized state {n}"),
        }
    }

    pub fn is_decision_state(&self) -> bool {
        matches!(
            self,
            State::SelectingHand | State::PackOpening | State::Shop | State::BlindSelect
        )
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::BlindSelect => write!(f, "Blind Select"),
            State::Tutorial => write!(f, "Tutorial"),
            State::SelectingHand => write!(f, "Selecting Hand"),
            State::PlayTarot => write!(f, "Play Tarot"),
            State::HandPlayed => write!(f, "Hand Played"),
            State::RoundEval => write!(f, "Round Eval"),
            State::DrawToHand => write!(f, "Draw To Hand"),
            State::Shop => write!(f, "Shop"),
            State::MainMenu => write!(f, "Main Menu"),
            State::GameOver => write!(f, "Game Over"),
            State::NewRound => write!(f, "New Round"),
            State::Splash => write!(f, "Splash Screen"),
            State::RedeemCustomVoucher => write!(f, "Redeeming Custom Voucher"),
            State::PackOpening => write!(f, "Pack Opening"),
            State::Sandbox => write!(f, "Sandbox"),
        }
    }
}
