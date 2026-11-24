use reqwest::{Client, Error, Response};
use serde_json::{Value, json};

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
