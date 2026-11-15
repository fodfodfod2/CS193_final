use std::vec;

use crate::systemone_api::*;

mod balatro_state;
mod systemone_api;

fn main() {
    println!("{}", balatro_state::parsing::read_game_state().unwrap());

    let a = ApiChoice::new("Red", "The thing is red");
    let b = ApiChoice::new("Blue", "The thing is blue");

    let qs = vec![a, b];
    let aqc = ApiQuestionCriteria::Choice(qs);

    let aq = ApiQuestion::new("Q1", "What color are apples?", aqc);
    let ar = ApiRequest::new("Answer the question", "jev-latest", vec![aq]);
    let rt = tokio::runtime::Runtime::new().unwrap();

    // 2. Block the current thread and drive the async function to completion
    rt.block_on(async { ar.get_response(ApiType::Jev).await });
}
