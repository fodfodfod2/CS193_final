#[cfg(test)]
mod rearrange_tests {
    use super::*;

    #[tokio::test]
    async fn test_rearrange_hand() {
        rearrange_hand(&vec![1i16, 0i16]).await.unwrap();
    }

    #[tokio::test]
    async fn test_rearrange_jokers() {
        rearrange_jokers(&vec![1i16, 0i16]).await.unwrap();
    }

    #[tokio::test]
    async fn test_rearrange_consumeables() {
        rearrange_consumables(&vec![1i16, 0i16]).await.unwrap();
    }
}

#[cfg(test)]
mod play_tests {
    use super::*;

    #[tokio::test]
    async fn test_play_hand() {
        play_hand(&vec![1i16, 0i16]).await.unwrap();
    }

    #[tokio::test]
    async fn test_discard_hand() {
        discard_hand(&vec![0i16, 1i16]).await.unwrap();
    }
}

#[cfg(test)]
mod shop_tests {
    use super::*;

    #[tokio::test]
    async fn test_buy_hand() {
        buy_card(0).await.unwrap();
    }

    #[tokio::test]
    async fn test_buy_voucher() {
        buy_voucher(0).await.unwrap();
    }

    #[tokio::test]
    async fn test_buy_pack() {
        buy_pack(0).await.unwrap();
    }

    #[tokio::test]
    async fn test_leave_shop() {
        leave_shop().await.unwrap();
    }

    #[tokio::test]
    async fn test_reroll_shop() {
        reroll_shop().await.unwrap();
    }
}

#[cfg(test)]
mod open_pack_tests {
    use super::*;

    #[tokio::test]
    async fn test_open_pack() {
        select_pack_card(0, &Vec::<i16>::new(), false)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_skip_pack() {
        select_pack_card(0, &Vec::<i16>::new(), true).await.unwrap();
    }
}

#[cfg(test)]
mod blind_select_tests {
    use super::*;

    #[tokio::test]
    async fn test_next_blind() {
        next_blind().await.unwrap()
    }

    #[tokio::test]
    async fn test_skip_blind() {
        skip_blind().await.unwrap()
    }
}

#[cfg(test)]
mod sell_item_tests {
    use super::*;

    #[tokio::test]
    async fn test_sell_joker() {
        sell_joker(0).await.unwrap()
    }

    #[tokio::test]
    async fn test_sell_consumable() {
        sell_consumable(0).await.unwrap()
    }
}
