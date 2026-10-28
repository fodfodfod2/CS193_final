local ranks = {
  ["14"] = 14, -- ACE
  ["2"] = 2,
  ["3"] = 3,
  ["4"] = 4,
  ["5"] = 5,
  ["6"] = 6,
  ["7"] = 7,
  ["8"] = 8,
  ["9"] = 9,
  ["10"] = 10,
  ["11"] = 11, -- Jack
  ["12"] = 12, -- Queen
  ["13"] = 13, -- King
}

local suits = {
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
  ["Base"] = {"",""},
  ["Bonus card"] = {"bonus", "+30 chips when scored"},
  ["Mult card"] = {"mult", "+4 mult when scored"},
  ["Wild card"] = {"wild", "counts as any suit"},
  ["Glass card"] = {"glass", "*2 mult, 1 in 4 chance to be destroyed when triggered"},
  ["Steel card"] = {"steel", "*1.5 mult if held in hand when a hand is scored"},
  ["Stone card"] = {"stone", "+50 chips when scored. Always scores"},
  ["Gold card"] = {"gold", "+3$ if held in hand at the end of a round"},
  ["Lucky card"] = {"lucky", "1 in 5 chance for +20 mult, 1 in 15 chance for +20$"},
}

local facing = {
  ["front"] = false,
  ["back"] = true,
}

local function enhancement(e)
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
    ["RANK"] = c.base.id,
    ["ENHANCEMENT"] = enhancement(c.ability.effect),
    ["SEAL"] = seal(c.seal),
    ["DISABLED"] = c.debuff,
    ["FACE_DOWN"] = facing[c.facing],
  }
end
