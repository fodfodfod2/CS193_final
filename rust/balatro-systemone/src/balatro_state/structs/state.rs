use super::*;

/* This represents possible game states */
pub enum State {
    BlindSelect,
    Tutorial,
    SelectingHand,
    PlayTarot,
    HandPlayed,
    RoundEval,
    DrawToHand,
    Shop,
    TarotPack,
    PlanetPack,
    SpectralPack,
    StandardPack,
    BuffPack,
    MainMenu,
    GameOver,
    NewRound,
    Splash,
    RedeemCustomVoucher,
    CustomPack,
    Sandbox,
    Other,
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
            9 => State::TarotPack,
            10 => State::PlanetPack,
            11 => State::MainMenu,
            12 => State::Tutorial,
            13 => State::Splash,
            14 => State::Sandbox,
            15 => State::SpectralPack,
            17 => State::StandardPack,
            18 => State::BuffPack,
            19 => State::NewRound,
            998 => State::RedeemCustomVoucher,
            999 => State::CustomPack,
            _ => State::Other,
        }
    }

    pub fn is_decision_state(&self) -> bool {
        matches!(
            self,
            State::SelectingHand
                | State::TarotPack
                | State::PlanetPack
                | State::SpectralPack
                | State::StandardPack
                | State::BuffPack
                | State::Shop
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
            State::TarotPack => write!(f, "TarotP ack"),
            State::PlanetPack => write!(f, "Planet Pack"),
            State::SpectralPack => write!(f, "Spectral Pack"),
            State::StandardPack => write!(f, "Standard Pack"),
            State::BuffPack => write!(f, "Buffoon Pack"),
            State::MainMenu => write!(f, "Main Menu"),
            State::GameOver => write!(f, "Game Over"),
            State::NewRound => write!(f, "New Round"),
            State::Splash => write!(f, "Splash Screen"),
            State::RedeemCustomVoucher => write!(f, "Redeeming Custom Voucher"),
            State::CustomPack => write!(f, "Custom Pack Opening"),
            State::Sandbox => write!(f, "Sandbox"),
            _ => write!(f, "STATE_NOT_RECOGNIZED"),
        }
    }
}
