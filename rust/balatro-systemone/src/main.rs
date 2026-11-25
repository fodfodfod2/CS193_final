use crate::{balatro_state::get_claude_gen_prompt, game_interface::*, systemone_api::*};

mod balatro_state;
mod game_interface;
mod systemone_api;

#[tokio::main]
async fn main() {
    let gs = balatro_state::parsing::read_game_state().unwrap();
    println!("{}", gs);
    let acts = gs.get_possible_actions();
    if let Some(apc) = acts {
        println!("{apc}");
        let state = &format!("{}\n\n\n{}", get_claude_gen_prompt(), gs);
        let ar = ApiRequest::new(state, "jev-latest", vec![apc]);
        let ans = ar.get_response(ApiType::Jev).await;
        if let ApiAnswer::Choice(_, _, a) = ans[0] {
            a.perform(&gs, state).await.unwrap();
        }
    }

    // let a = ApiChoice::new("Red", "The thing is red");
    // let b = ApiChoice::new("Blue", "The thing is blue");

    // let qs = vec![a, b];
    // let aqc = ApiQuestionCriteria::Choice(qs);

    // let aq = ApiQuestion::new("Q1", "What color are apples?", aqc);
}
