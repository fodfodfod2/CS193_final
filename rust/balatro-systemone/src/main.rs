mod balatro_state;
mod systemone_api;

fn main() {
    println!("{}", balatro_state::parsing::read_game_state().unwrap());
}
