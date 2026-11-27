use std::{thread::sleep, time::Duration};

use crate::{balatro_state::get_claude_gen_prompt, game_interface::*, systemone_api::*};

mod balatro_state;
mod game_interface;
mod systemone_api;

/* This is how many 1 sec breaks the game will take before breaking */
const COUNTER_MAX: usize = 15;

#[tokio::main]
async fn main() {
    game_loop().await;
}

async fn game_loop() {
    let api_type = match std::env::var("USE_LAYA") {
        Ok(s) => match s.as_str() {
            "1" => ApiType::Laya,
            _ => ApiType::Jev,
        },
        Err(_) => ApiType::Jev,
    };

    let mut counter = COUNTER_MAX;
    while counter > 0 {
        let gs_opt = balatro_state::parsing::read_game_state();
        if let Some(gs) = gs_opt {
            println!("{}", gs);

            let acts = gs.get_possible_actions();
            if let Some(apc) = acts {
                counter = COUNTER_MAX;
                println!("{apc}");
                let state = &format!("{}\n\n\n{}", get_claude_gen_prompt(), gs);
                let ar = ApiRequest::new(state, api_type.get_model_name(), vec![apc]);
                let ans = ar.get_response(api_type).await;
                if let ApiAnswer::Choice(_, _, a) = ans[0] {
                    a.perform(&gs, state, api_type).await.unwrap();
                }
            } else {
                counter -= 1;
                sleep(Duration::from_millis(1000));
            }
        } else {
            counter -= 1;
            sleep(Duration::from_millis(1000));
        }
    }
}
