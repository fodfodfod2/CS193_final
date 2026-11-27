use core::panic;

use crate::{balatro_state::get_claude_gen_prompt, game_interface::*, systemone_api::*};

mod balatro_state;
mod game_interface;
mod systemone_api;

#[tokio::main]
async fn main() {
    let api_type = match std::env::var("USE_LAYA") {
        Ok(s) => match s.as_str() {
            "1" => ApiType::Laya,
            _ => ApiType::Jev,
        },
        Err(_) => ApiType::Jev,
    };

    let gs = balatro_state::parsing::read_game_state().unwrap();
    println!("{}", gs);
    let acts = gs.get_possible_actions();
    if let Some(apc) = acts {
        println!("{apc}");
        let state = &format!("{}\n\n\n{}", get_claude_gen_prompt(), gs);
        let ar = ApiRequest::new(state, api_type.get_model_name(), vec![apc]);
        let ans = ar.get_response(api_type).await;
        if let ApiAnswer::Choice(_, _, a) = ans[0] {
            a.perform(&gs, state, api_type).await.unwrap();
        }
    }
}
