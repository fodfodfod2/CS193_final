use std::vec;

use reqwest::{Client, Error, Response};
use serde_json::{Value, json};

use crate::{
    ApiAnswer, ApiChoice, ApiQuestion,
    ApiQuestionCriteria::{self},
    ApiRequest, ApiType,
    balatro_state::{Card, GameState, vec_to_str},
};

/*
 * This crate represents the interface with the game,
 * using the balatrobot mod/api.
 * Primary usage should be through Action and Action.perform()
 */


/* Helper function to send a payload to the balatrobot api */
async fn send_payload(payload: Value) -> Result<Response, Error> {
    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:12346")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    Ok(res)
}

/* This represents the areas in the shop that can be bought from */
enum ShopItem {
    Card,
    Voucher,
    Pack,
}

/* This will buy an item from the given area with the given idx */
async fn buy(item: ShopItem, idx: i16) -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "buy",
        "params" : {
            match item {
                ShopItem::Card => "card",
                ShopItem::Voucher => "voucher",
                ShopItem::Pack => "pack"
            } : idx},
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

/* Buy an item from the card area with the given idx */
pub async fn buy_card(idx: i16) -> Result<(), Error> {
    buy(ShopItem::Card, idx).await
}

/* Buy an item from the voucher area with the given idx */
pub async fn buy_voucher(idx: i16) -> Result<(), Error> {
    buy(ShopItem::Voucher, idx).await
}

/* Buy an item from the pack area with the given idx */
pub async fn buy_pack(idx: i16) -> Result<(), Error> {
    buy(ShopItem::Pack, idx).await
}

/* reroll the shop */
pub async fn reroll_shop() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "reroll",
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

/*
 * Select a card from the given pack,
 * if skip == true, then skip the pack and ignore everything else
 * targets represents the targets of the picked card (such as targeted by a
 * tarot card)
 */
pub async fn select_pack_card(idx: i16, targets: &Vec<i16>, skip: bool) -> Result<(), Error> {
    if skip {
        let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "pack",
        "params" : {"skip" : true},
        "id" : 1});
        send_payload(payload).await?;
        return Ok(());
    }

    let params = match !targets.is_empty() {
        true => json!({
            "card" : idx,
            "targets" : targets
        }),
        _ => json!({"card" : idx}),
    };

    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "pack",
        "params" : params,
        "id" : 1});
    send_payload(payload).await?;

    Ok(())
}

/* Leave the shop */
pub async fn leave_shop() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "next_round",
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

/* Different plays you can make from your hand */
enum HandUse {
    Discard,
    Play,
}

/* This function will perform a handuse on the given card idxs */
async fn use_hand(action: HandUse, cards: &Vec<i16>, gs: &GameState) -> Result<(), Error> {
    let hand_size = gs.get_hand().len();
    let mut hand_rearrange = cards.clone();
    for n in 0..hand_size {
       if !hand_rearrange.contains(&(n as i16)) {
            hand_rearrange.push(n as i16);
        }
    }
    rearrange_hand(&hand_rearrange).await.unwrap();

    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : match action {
            HandUse::Discard => "discard",
            HandUse::Play => "play",
        },
        "params" : {"cards" : (0..cards.iter().len()).collect::<Vec<usize>>()},
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

/* Play the cards with the given idxs */
pub async fn play_hand(cards: &Vec<i16>, gs: &GameState) -> Result<(), Error> {
    use_hand(HandUse::Play, cards, gs).await
}

/* Discard the cards with the given idxs */
pub async fn discard_hand(cards: &Vec<i16>, gs: &GameState) -> Result<(), Error> {
    use_hand(HandUse::Discard, cards, gs).await
}

/* The type of cards that can be rearranged */
enum CardType {
    Hand,
    Jokers,
    Consumeables,
}

/* Rearrange a certain area of cards in hand */
async fn rearrange_cards(card_type: CardType, targets: &Vec<i16>) -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "rearrange",
        "params" : {
            match card_type {
                CardType::Hand => "hand",
                CardType::Jokers => "jokers",
                CardType::Consumeables => "consumables",
            } : targets
        },
        "id" : 1
    });
    send_payload(payload).await?;
    Ok(())
}

/*
 * Rearrange the hand. Targets should contain the full amount of cards
 * ex: [0,1,2,4,3] to swap cards 5 and 4 in a 5 card hand
 */
pub async fn rearrange_hand(targets: &Vec<i16>) -> Result<(), Error> {
    rearrange_cards(CardType::Hand, targets).await
}

/*
 * Rearrange the jokers. Targets should contain the full amount of jokers
 * ex: [0,1,2,4,3] to swap jokers 5 and 4 with 5 jokers
 */
pub async fn rearrange_jokers(targets: &Vec<i16>) -> Result<(), Error> {
    rearrange_cards(CardType::Jokers, targets).await
}

/*
 * Rearrange the consumables area. Targets should contain the full amount
 * of consumables
 * ex: [0,2,1] to swap consumables 2 and 3 with 3 consumables
 */
pub async fn rearrange_consumables(targets: &Vec<i16>) -> Result<(), Error> {
    rearrange_cards(CardType::Consumeables, targets).await
}

/* Move onto the next blind */
pub async fn next_blind() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "select",
        "id" : 1
    });
    send_payload(payload).await?;
    Ok(())
}

/* Skip the next blind */
pub async fn skip_blind() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "skip",
        "id" : 1
    });
    send_payload(payload).await?;
    Ok(())
}

/* Which kinds of items can be sold */
enum SellItem {
    Joker,
    Consumable,
}

/* Sell an item of a certain type and of a given idx */
async fn sell(item_type: SellItem, idx: i16) -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "sell",
        "params" : {
            match item_type {
                SellItem::Joker => "joker",
                SellItem::Consumable => "consumable",
            } : idx,
        },
        "id" : 1
    });
    send_payload(payload).await?;
    Ok(())
}

/* Sell the joker of a given idx */
pub async fn sell_joker(idx: i16) -> Result<(), Error> {
    sell(SellItem::Joker, idx).await
}

/* Sell the consumable of a given idx */
pub async fn sell_consumable(idx: i16) -> Result<(), Error> {
    sell(SellItem::Consumable, idx).await
}

/* Collect cash after beating a blind */
pub async fn cash_out() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "cash_out",
        "id" : 1,
    });
    send_payload(payload).await?;
    Ok(())
}

/* Use a consumable you own */
pub async fn use_item(idx: i16, targets: Vec<i16>) -> Result<(), Error> {
    let payload = match targets.is_empty() {
        true => json!({
            "jsonrpc" : "2.0",
            "method" : "use",
            "params" : {
                "consumable" : idx,
            },
            "id" : 1,
        }),
        false => json!({
            "jsonrpc" : "2.0",
            "method" : "use",
            "params" : {
                "consumable" : idx,
                "cards" : targets,
            },
            "id" : 1,
        }),
    };

    send_payload(payload).await?;
    Ok(())
}

/* This represents possible actions the player can take */
#[derive(Clone, Copy)]
pub enum Action {
    NextBlind,
    SkipBlind,
    BuyCard(i16),
    BuyPack(i16),
    BuyVoucher(i16),
    UsePackItem(i16, (i8, i8)),
    SellJoker(i16),
    SellCons(i16),
    Reroll,
    EndRoundEval,
    LeaveShop,
    PlayHand,
    DiscardHand,
    RearrangeJokers,
    UseCons(i16, (i8, i8)),
    None,
}

impl Action {
    /*
     * Selected between min and max cards from from your hand
     * For each card, it will prompt the systemone model to choose the next
     * card. Returns a vec of the idx's of the cards
     */
    async fn select_hand_targets(
        prompt: &str,
        min: i8,
        max: i8,
        hand: &Vec<Card>,
        state: &str,
        api_type: ApiType,
    ) -> Result<Vec<i16>, Error> {
        let full_prompt = vec_to_str(
            &(String::from(prompt) + &format!(" \\nChoose between {min} and {max} targets")),
            hand,
        )
        .replace("\n", "\\n")
        .replace("\t", "\\t");
        let mut options = Vec::new();

        for i in 0..hand.len() {
            let c = &hand[i];
            options.push(ApiChoice::new(
                &format!("{i}"),
                &format!("Use card {i}: {c}"),
                Action::None,
            ));
        }

        let mut res = Vec::new();
        for i in 0..max {
            //if i == min {
            //    options.push(ApiChoice::new(
            //        "None",
            //        "Finish with the targets you have, generally this should be your last choice. Remember scoring more cards is typically better",
            //        Action::None,
            //    ));
            //}
            let apiqc = ApiQuestionCriteria::Choice(options.clone());
            let apiq = ApiQuestion::new(
                "hand target select",
                &(format!(
                    "you have chosen {i} targets so far. \
                    choose your next card, if applicable.\\n{}",
                    vec_to_str(
                        "Current Targets",
                        &res.iter().map(|&i| &hand[i as usize]).collect()
                    )
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                ) + &full_prompt),
                apiqc,
            );

            let apireq = ApiRequest::new(state, api_type.get_model_name(), vec![apiq]);
            let apires = apireq.get_response(api_type).await;
            let ans_str = match &apires[0] {
                ApiAnswer::Choice(s, _, _) => s.as_str(),
                _ => {
                    panic!("Somehow i got something other than a choice ans from a choice request")
                }
            };
            if ans_str == "None" {
                break;
            }

            let ans: i16 = ans_str.parse().unwrap();

            res.push(ans);
            for i in 0..options.len() {
                if ans_str == options[i].get_name() {
                    options.remove(i);
                    break;
                }
            }
        }

        Ok(res)
    }

    /* perform the Action */
    pub async fn perform(&self, gs: &GameState, state: &str, api_type: ApiType) -> Result<(), Error> {
        match self {
            Self::NextBlind => next_blind().await,
            Self::SkipBlind => skip_blind().await,
            Self::BuyCard(i) => buy_card(*i).await,
            Self::BuyPack(i) => buy_pack(*i).await,
            Self::EndRoundEval => cash_out().await,
            Self::BuyVoucher(i) => buy_voucher(*i).await,
            Self::UsePackItem(i, (min, max)) => select_pack_card(*i,
                    &Self::select_hand_targets(
                        &format!("Which cards do you want to target with {}?",
                            gs.get_pack_item(*i as usize)),
                        *min,
                        *max,
                        gs.get_hand(),
                        state,
                        api_type,
                    )
                    .await
                    .unwrap()
                , false).await,
            Self::SellJoker(i) => sell_joker(*i).await,
            Self::SellCons(i) => sell_consumable(*i).await,
            Self::Reroll => reroll_shop().await,
            Self::LeaveShop => leave_shop().await,
            Self::PlayHand => {
                println!("PLAYING");
                play_hand(
                    &Self::select_hand_targets(
                        "Which cards do you want to play?",
                        1,
                        5,
                        gs.get_hand(),
                        state,
                        api_type,
                    )
                    .await
                    .unwrap(),
                    gs,
                )
                .await
            }
            Self::DiscardHand => {
                discard_hand(
                    &Self::select_hand_targets(
                        "Which cards do you want to discard?, You have up to 5 cards, It is generally in your best interest to choose as many as possible, trying to form poker hands such as a straight, flush, pair etc. ",
                        1,
                        5,
                        gs.get_hand(),
                        state,
                        api_type,
                    )
                    .await
                    .unwrap(),
                    gs,
                )
                .await
            }
            Self::RearrangeJokers => Ok(()),
            Self::UseCons(i, (min, max)) => {
                use_item(
                    *i,
                    Self::select_hand_targets(
                        "Which cards do you want to discard? It is generally in your best interest to discard as many as possible (up to 5) in order to form poker hands such as a straight, flush, pair etc",
                        *min,
                        *max,
                        gs.get_hand(),
                        state,
                        api_type,
                    )
                    .await
                    .unwrap(),
                )
                .await
            }
            Self::None => panic!("None action was performed"),
        }
    }
}
