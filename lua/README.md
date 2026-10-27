# Goal
This is just a simple balatro mod to dump all of the data the rest of the mod
needs into an easy to parse table
## Table format
```
GAME
├──STATE:int // Represents the state of the game, google balatro states
├──HAND:[cards in hand] // This is only updated if the state is relevant
├──VOUCHERS_OWNED:[vouchers owned]
├──JOKERS_OWNED:[jokers owned]
├──CONSUMABLES // consumables owned
│   ├──PLANET:[arr of planets held]
│   ├──TAROT:[arr of tarot held]
│   └──SPECTRAL:[arr of spectral held]
├──SHOP // this contains items in the shop, only updated at new shops
│   │   // each of these items will be [item, price]
│   │   // for example PLANET:[[MARS,2],[JUPITER,3]]
│   ├──JOKER:[arr of jokers in shop]
│   ├──PLANET:[arr of planets in shop]
│   ├──TAROT:[arr of tarot in shop]
│   ├──CARD:[arr of cards in shop]
│   ├──PACKS:[arr of the packs offered in the shop]
│   └──VOUCHERS:[arr of the vouchers offered in the shop]
├──PACK // this will only be updated during pack openings
│   ├──TYPE:int // 0 - standard, 1 - planet, 2 - joker, 3 - tarot, 4 - spectral
│   ├──SELECT_NUM:int // the numer of items you can select
│   └──ITEMS:[arr of items in the pack]
├──BLINDS
│   ├──SMALL:long // the score needed for the small blind
│   ├──BIG:long // the score needed for the big blind
│   ├──BOSS:long // the score needed for the boss blind
│   ├──CURRENT:int // the current blind 0 if next blind is small, 1 big, 2 boss
│   └──BOSS_TYPE
│       ├──NAME:str // the name of the boss
│       └──DESC:str // the description of the boss
├──CTX
│   ├──HANDS:int // the amt of hands remaining
│   ├──DISCARDS:int // the amt of discards remaining
│   ├──MAX_HANDS:int
│   ├──MAX_DISCARDS:int
│   ├──HANDS_PLAYED:int // bit packed based on hand idx, played in current blind
│   └──MOST_PLAYED_HAND:int // the hand played the most throughout the run
├──HAND_LVL:[arr of hand levels indexed by hand type]
└──DECK:[cards in deck]
```
## OBJECT formats
```
CARD
├──RANK:int // the rank of the card 2-13
├──SUIT:int // the suit of the card 0=H, 1=D, 2=C, 3=S
├──ENHANCEMENT:ENHANCEMENT // the enhancement of the card
├──SEAL:SEAL // the seal on the card,
├──DISABLED:bool
└──FACE_DOWN:bool
```
```
JOKER
├──NAME:str // the name of the joker
├──DESC:str // the description of the joker
├──EDITION:EDITION // the edition of the joker,
├──SELL_VALUE:int // the sell value of the joker, 0 = unsellable
├──DISABLED:bool
└──STICKER:STICKER // the sticker on the joker,
```
```
VOUCHER
├──NAME:str // the name of the voucher
└──DESC:str // the desc of the voucher
```
```
PLANET
├──NAME:str // the name of the planet card
├──DESC:str // the desc of the planet card
├──HAND_ID:int // the idx of the hand that will be upgraded
└──SELL_VALUE:int // the sell value of the planet card
```
```
TAROT
├──NAME:str // the name of the tarot card
├──DESC:str // the desc of the tarot card
└──SELL_VALUE:int // the sell value of the tarot card
```
```
SPECTRAL
├──NAME:str // the name of the spectral card
├──DESC:str // the desc of the spectral card
└──SELL_VALUE:int // the sell value of the spectral card
```
```
STICKER
├──NAME:str // the name of the joker sticker
└──DESC:str // the desc of the joker sticker
```
```
EDITION
├──NAME:str // the name of the joker edition
└──DESC:str // the desc of the joker edition
```
```
SEAL
├──NAME:str // the name of the card seal
└──DESC:str // the desc of the card seal
```
```
ENHANCEMENT
├──NAME:str // the name of the card enhancement
└──DESC:str // the desc of the card enhancement
```

