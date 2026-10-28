local suits = {
  ["Ace"] = 1,
  ["2"] = 2,
  ["3"] = 3,
  ["4"] = 4,
  ["5"] = 5,
  ["6"] = 6,
  ["7"] = 7,
  ["8"] = 8,
  ["9"] = 9,
  ["10"] = 10,
  ["Jack"] = 11,
  ["Queen"] = 12,
  ["King"] = 13,
}

local ranks = {
  ["Hearts"] = 0,
  ["Diamonds"] = 1,
  ["Clubs"] = 2,
  ["Spades"] = 3,
}

local seals = {
  ["Gold"] = {"Gold", "Gain +3$ when scored"},
  ["Red"] = {"Red", "Retrigger this card 1 time"},
  ["Blue"] = {"Blue", "Creates the Planet card for final played poker hand of round if held in hand"},
  ["Purple"] = {"Purple","Creates a random Tarot card when discarded"},
}

local enhancements = {
  ["bonus card"] = {"bonus", "+30 chips when scored"},
  ["mult card"] = {"mult", "+4 mult when scored"},
  ["wild card"] = {"wild", "counts as any suit"},
  ["glass card"] = {"glass", "*2 mult, 1 in 4 chance to be destroyed when triggered"},
  ["steel card"] = {"steel", "*1.5 mult if held in hand when a hand is scored"},
  ["stone card"] = {"stone", "+50 chips when scored. Always scores"},
  ["gold card"] = {"gold", "+3$ if held in hand at the end of a round"},
  ["lucky card"] = {"lucky", "1 in 5 chance for +20 mult, 1 in 15 chance for +20$"},
}

local facing = {
  ["front"] = false,
  ["back"] = true,
}

local function enhancemet(e)
  if e == nil then
    return {"",""}
  end
  return enhancements[e]
end

local function seal(s)
  if s == nil then
    return {"",""}
  end
  return seals[s]
end


function card(c)
  if c == nil or c.base == nil then
    return nil
  end
  return {
    ["SUIT"] = suits[c.base.suit],
    ["RANK"] = ranks[c.base.rank],
    ["ENHANCEMENT"] = enhancemet(c.ability.effect),
    ["SEAL"] = seal(c.seal),
    ["DISABLED"] = c.debuff,
    ["FACE_DOWN"] = facing[c.facing],
  }
end
