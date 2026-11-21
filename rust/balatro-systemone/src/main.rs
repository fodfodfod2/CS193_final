use crate::systemone_api::*;

mod balatro_state;
mod systemone_api;

fn main() {
    let gs = balatro_state::parsing::read_game_state().unwrap();
    println!("{}", gs);
    let acts = gs.get_possible_actions();
    if let Some(apc) = acts {
        println!("{apc}");


        let ar = ApiRequest::new("Answer the question", "jev-latest", vec![apc]);
        let rt = tokio::runtime::Runtime::new().unwrap();
    
        rt.block_on(async { ar.get_response(ApiType::Jev).await });
    }

    // let a = ApiChoice::new("Red", "The thing is red");
    // let b = ApiChoice::new("Blue", "The thing is blue");

    // let qs = vec![a, b];
    // let aqc = ApiQuestionCriteria::Choice(qs);

    // let aq = ApiQuestion::new("Q1", "What color are apples?", aqc);
}
