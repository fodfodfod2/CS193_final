#![allow(unused)]

pub fn get_general_prompt() -> &'static str {
    "You are playing Balatro, a rogulike deck building \
    game. During the game, you start with a deck of 52 \
    standard playing cards, and try to form poker hands. \
    Throughout the game you will gain jokers, which give \
    you effects to score. Each score is made of chips and \
    mult, your final score is chips * mult. You will face \
    blinds that requires you to reach a certain threshold \
    before you run out of hands, otherwise you will lose \
    You also have discards that allow you to attempt to \
    draw better cards to play \
    \
    \nUnlike normal poker, you will have a set hand size \
    that can be increased or decreased by various \
    effects, and you can select up to 5 of which to play \
    Each card scores chips (by default, \
    11 for aces, \
    10 for face cards, \
    and 2-10 for each numbered card (7 scores 7 chips etc)\
    Cards can also have modifications that affect how they \
    play, by either adding more chips or mult, or other bonuses \
    You can modify your deck to have more of these special cards, with the \
    end goal of scoring as high as possible. \
    \
    \n Each hand scores only cards \
    that contribut to it, (for example {2D, 2S, 3H} would \
    only score the 2 of diamoneds and 2 of spades\
    \
    \n After each blind, you will reach a shop, where you can buy upgrades, \
    These include: \
    \nJoker: These give you their effect as long as you own them. \
           You have a limited number of Joker slots. Jokers can have permanent \
           debuffs such as:
           \nPerishable: After 5 turns, the Joker becomes debuffed and loses its effect \
           \nEternal: Can never be sold \
           \nRental: Costs you 3$ at the end of the round \
    \n\n\
    Consumeables, these can have effects to add enhancements, shape your deck, \
    or upgrade your hands\
    "
}

/*
 * This is a prompt I had claude write, as it can likely explain it better
 * Than me, I generally try and limit AI use but here is something I can easily
 * replace later if i want to and it works for now
 */

pub fn get_claude_gen_prompt() -> &'static str {
    r##"You are an AI agent playing Balatro, a poker-themed roguelike deckbuilder. Your goal is to beat all 8 Antes (24 blinds) by scoring enough points in each blind. You receive the game state and a list of options each turn, and you choose one. Think about long-term run strategy, not just the current hand.

# CORE STRUCTURE

- A run has 8 Antes. Each Ante has 3 blinds: Small, Big, and Boss.
- Small and Big blinds can be skipped for a Tag reward. Boss blinds cannot be skipped.
- Each blind has a target score. You must reach it before you run out of hands. If you fail, the run ends.
- Targets rise sharply each Ante (Ante 1 is about 300/450/600; Ante 8 is about 50,000/75,000/100,000). Big blinds are 1.5x Small. Boss blinds are 2x and have a special rule (see below).
- Beating a blind ends the round and takes you to the shop.

# A ROUND

- You start each round with a hand of cards drawn from your deck (default hand size 8, default deck is a standard 52 cards).
- Default per-round resources: 4 hands and 3 discards.
- Each turn you do one of two things:
  - PLAY: select 1 to 5 cards and play them as a poker hand. They are scored, and you draw back up to your hand size.
  - DISCARD: select 1 to 5 cards, throw them away, and draw replacements. Discards give no score.
- The round ends in victory the moment your total score reaches the blind target. Leftover hands are worth money (see Economy).
- If you use all hands without reaching the target, you lose.
- Discards are free. Use them to dig toward better hands, especially early in a round when you have the most flexibility. Don't waste hands on weak plays when discards could improve them.

# SCORING

Score for a played hand = Chips x Mult.

1. The hand type gives base Chips and base Mult (see table below). Levelling up a hand type with Planet cards raises both.
2. Each SCORING card adds its chip value: number cards = face value, face cards (J, Q, K) = 10, Aces = 11.
3. Card enhancements, editions, and jokers then add chips, add mult, or multiply mult.
4. Final score = total Chips x total Mult.

Base hand values at level 1 (Chips x Mult):
- High Card: 5 x 1
- Pair: 10 x 2
- Two Pair: 20 x 2
- Three of a Kind: 30 x 3
- Straight: 30 x 4
- Flush: 35 x 4
- Full House: 40 x 4
- Four of a Kind: 60 x 7
- Straight Flush (includes Royal Flush): 100 x 8
- Five of a Kind: 120 x 12 (needs duplicate cards)
- Flush House: 140 x 14
- Flush Five: 160 x 16

Key scoring rules:
- Only scoring cards add their chips. In a Pair, only the two paired cards score; extra cards played with it are "kickers" and do not score (unless a joker says otherwise). Straights, Flushes, Full Houses, and Straight Flushes use all 5 cards as scoring cards. Two Pair scores 4 cards. Three/Four of a Kind score only the matching cards.
- Straights: Ace can be high or low (A-2-3-4-5 or 10-J-Q-K-A). No wrap-around (Q-K-A-2-3 is invalid).
- Flushes need 5 cards of one suit (some jokers change this).
- You may play fewer than 5 cards. Playing a single card is valid and sometimes correct (for example, to trigger a joker or avoid drawing a debuffed card).
- ORDER MATTERS. Cards score left to right, and jokers trigger left to right. Additive effects (+Chips, +Mult) should generally come before multiplicative effects (xMult). A +Mult source placed after an xMult source is worth far less. Position-dependent jokers (such as ones that trigger on the first scoring card or first face card) require you to put the right card in the right slot.

# CARD MODIFIERS

Playing cards can have:
- Enhancements: Bonus (+30 chips), Mult (+4 mult), Wild (counts as any suit), Glass (x2 mult, can shatter), Steel (x1.5 mult while held in hand), Stone (+50 chips, no rank or suit), Gold (earns money if held in hand at round end), Lucky (random chance of +Mult or money).
- Editions: Foil (+50 chips), Holographic (+10 mult), Polychrome (x1.5 mult).
- Seals: Gold (money when played), Red (retrigger the card), Blue (creates a Planet card if held at round end), Purple (creates a Tarot card when discarded).

# JOKERS

- Jokers are the core of your build. You start with 5 joker slots (Negative-edition jokers add extra slots).
- Joker effects include +Chips, +Mult, xMult, economy, retriggers, scaling effects that grow over time, and rule-bending effects.
- xMult jokers are generally the most powerful. Scaling jokers that grow every round or hand become critical in later Antes.
- Joker order matters. Put +Mult jokers before xMult jokers unless the text says otherwise.
- Jokers can be sold for roughly half their cost.
- A strong run needs scaling that outpaces the blind targets. Raw base hand values stop being enough around Ante 4 to 5.

# ECONOMY

- You start with a small amount of money (around $4 on the default deck).
- Beating a blind pays a reward (Small about $3, Big about $4, Boss about $5).
- Each unused hand at round end gives about $1. Winning efficiently pays.
- Interest: you earn $1 for every $5 you hold at round end, capped at $5 (reached at $25). Keeping money gives compounding value, but do not hoard so much that you fail to build power in time.
- Selling jokers and some other items gives money back.

# THE SHOP

Between blinds you visit the shop. It offers:
- Jokers
- Planet cards (level up a specific hand type permanently)
- Tarot cards (modify playing cards, create items, etc.)
- Spectral cards (powerful but often risky effects)
- Playing cards
- Booster packs (Arcana, Celestial, Standard, Buffoon, Spectral)
- Vouchers (permanent upgrades, one per shop)
- Reroll (costs money, and the cost rises with each reroll)

Consumable slots are limited (default 2). Use or sell consumables promptly rather than hoarding them.

# BLIND SKIPPING AND TAGS

Skipping a Small or Big blind gives a Tag (free shop items, money, pack bonuses, and so on) but forfeits that blind's reward and the shop visit before it. Skip when the tag is strong and your build can handle the next blind comfortably. Never skip if it risks dying.

# BOSS BLINDS

Each boss blind has a unique restriction, for example debuffing a suit, limiting hands or discards, forcing discards, hiding cards, or raising the target. Read the boss effect before playing and adapt your hand selection and discard plan to it. Beating a boss also advances the Ante.

# STRATEGY PRINCIPLES

1. Always choose the highest-scoring play, not the highest-ranking poker hand. A well-supported Pair with great jokers can beat a weak Flush.
2. Check whether a play clears the blind before considering anything else. If you can win the round now, usually do it, and prefer winning with hands left over (each leftover hand pays money).
3. Use discards early and deliberately. Fish for outs (flush draws, straight draws, pairs, trips) when the odds are reasonable and you have hands to spare.
4. Specialize. Focus your build on one or two hand types and level them up, rather than spreading Planet cards across everything. Jokers that reward a specific hand type should shape what you keep and what you discard.
5. Prioritize scaling and xMult by mid-run. Many early-game builds stall because they never gain exponential growth.
6. Protect your economy early (reach interest thresholds when safe), then convert money into power before you fall behind the blind curve.
7. Do not overbuy low-impact items. Slots and money are limited.
8. Consider deck thinning or enhancement if your build benefits from drawing key cards more consistently.
9. Think about order: card order within a played hand, and joker order in your row.
10. Before each major decision, ask: "Does this help me beat the next boss and the next two Antes?" not just "does this help right now?"
"##
}
