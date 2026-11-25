use std::fmt::format;

use reqwest::{Client, Error, Response};
use serde_json::{Value, json};

use crate::{
    ApiAnswer, ApiChoice, ApiQuestion,
    ApiQuestionCriteria::{self},
    ApiRequest, ApiType,
    balatro_state::{Card, GameState, vec_to_str},
};

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

enum ShopItem {
    Card,
    Voucher,
    Pack,
}

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

pub async fn buy_card(idx: i16) -> Result<(), Error> {
    buy(ShopItem::Card, idx).await
}

pub async fn buy_voucher(idx: i16) -> Result<(), Error> {
    buy(ShopItem::Voucher, idx).await
}

pub async fn buy_pack(idx: i16) -> Result<(), Error> {
    buy(ShopItem::Pack, idx).await
}

pub async fn reroll_shop() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "reroll",
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

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

pub async fn leave_shop() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "next_round",
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

enum HandUse {
    Discard,
    Play,
}
async fn use_hand(action: HandUse, cards: &Vec<i16>) -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : match action {
            HandUse::Discard => "discard",
            HandUse::Play => "play",
        },
        "params" : {"cards" : cards},
        "id" : 1});
    send_payload(payload).await?;
    Ok(())
}

pub async fn play_hand(cards: &Vec<i16>) -> Result<(), Error> {
    use_hand(HandUse::Play, cards).await
}

pub async fn discard_hand(cards: &Vec<i16>) -> Result<(), Error> {
    use_hand(HandUse::Discard, cards).await
}

enum CardType {
    Hand,
    Jokers,
    Consumeables,
}

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

pub async fn rearrange_hand(targets: &Vec<i16>) -> Result<(), Error> {
    rearrange_cards(CardType::Hand, targets).await
}

pub async fn rearrange_jokers(targets: &Vec<i16>) -> Result<(), Error> {
    rearrange_cards(CardType::Jokers, targets).await
}

pub async fn rearrange_consumables(targets: &Vec<i16>) -> Result<(), Error> {
    rearrange_cards(CardType::Consumeables, targets).await
}

pub async fn next_blind() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "select",
        "id" : 1
    });
    send_payload(payload).await?;
    Ok(())
}

pub async fn skip_blind() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "skip",
        "id" : 1
    });
    send_payload(payload).await?;
    Ok(())
}

enum SellItem {
    Joker,
    Consumable,
}

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

pub async fn sell_joker(idx: i16) -> Result<(), Error> {
    sell(SellItem::Joker, idx).await
}

pub async fn sell_consumable(idx: i16) -> Result<(), Error> {
    sell(SellItem::Consumable, idx).await
}

pub async fn cash_out() -> Result<(), Error> {
    let payload = json!({
        "jsonrpc" : "2.0",
        "method" : "cash_out",
        "id" : 1,
    });
    send_payload(payload).await?;
    Ok(())
}

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
    LeaveShop,
    PlayHand,
    DiscardHand,
    RearrangeJokers,
    UseCons(i16, (i8, i8)),
    None,
}

impl Action {
    async fn select_hand_targets(
        prompt: &str,
        min: i8,
        max: i8,
        hand: &Vec<Card>,
        state: &str,
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
                &format!("Target card {i}: {c}"),
                Action::None,
            ));
        }

        let mut res = Vec::new();
        for i in 0..max {
            if i == min {
                options.push(ApiChoice::new(
                    "None",
                    "Finish with the targets you have",
                    Action::None,
                ));
            }
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

            let apireq = ApiRequest::new(state, "jev-latest", vec![apiq]);
            let apires = apireq.get_response(ApiType::Jev).await;
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

    pub async fn perform(&self, gs: GameState, state: &str) -> Result<(), Error> {
        match self {
            Self::NextBlind => next_blind().await,
            Self::SkipBlind => skip_blind().await,
            Self::BuyCard(i) => buy_card(*i).await,
            Self::BuyPack(i) => buy_pack(*i).await,
            Self::BuyVoucher(i) => buy_voucher(*i).await,
            Self::UsePackItem(i, (min, max)) => Ok(()),
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
                    )
                    .await
                    .unwrap(),
                )
                .await
            }
            Self::DiscardHand => {
                discard_hand(
                    &Self::select_hand_targets(
                        "Which cards do you want to discard?",
                        1,
                        5,
                        gs.get_hand(),
                        state,
                    )
                    .await
                    .unwrap(),
                )
                .await
            }
            Self::RearrangeJokers => Ok(()),
            Self::UseCons(i, (min, max)) => {
                use_item(
                    *i,
                    Self::select_hand_targets(
                        "Which cards do you want to discard?",
                        *min,
                        *max,
                        gs.get_hand(),
                        state,
                    )
                    .await
                    .unwrap(),
                )
                .await
            }
            Self::None => Ok(()),
        }
    }
}
