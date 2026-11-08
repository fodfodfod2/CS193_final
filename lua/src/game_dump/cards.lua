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
  ["Gold"] = {NAME = "Gold", DESC = "Gain +3$ when scored"},
  ["Red"] = {NAME = "Red", DESC = "Retrigger this card 1 time"},
  ["Blue"] = {NAME = "Blue", DESC = "Creates the Planet card for final played poker hand of round if held in hand"},
  ["Purple"] = {NAME = "Purple", DESC = "Creates a random Tarot card when discarded"},
}

local enhancements = {
  ["Base"] = {NAME = "", DESC = ""},
  ["Bonus Card"] = {NAME = "bonus", DESC = "+30 chips when scored"},
  ["Mult Card"] = {NAME = "mult", DESC = "+4 mult when scored"},
  ["Wild Card"] = {NAME = "wild", DESC = "counts as any suit"},
  ["Glass Card"] = {NAME = "glass", DESC = "*2 mult, 1 in 4 chance to be destroyed when triggered"},
  ["Steel Card"] = {NAME = "steel", DESC = "*1.5 mult if held in hand when a hand is scored"},
  ["Stone Card"] = {NAME = "stone", DESC = "+50 chips when scored. Always scores"},
  ["Gold Card"] = {NAME = "gold", DESC = "+3$ if held in hand at the end of a round"},
  ["Lucky Card"] = {NAME = "lucky", DESC = "1 in 5 chance for +20 mult, 1 in 15 chance for +20$"},
}

local facing = {
  ["front"] = false,
  ["back"] = true,
}

local function enhancement(card)
  e = card.ability.effect
  if e == nil then
    return {"",""}
  end

  if enhancements[e] == nil then
    sendDebugMessage("failed to read enhancement "..e)
  end
  return enhancements[e]
end

local function seal(card)
  s = card.seal
  if s == nil then
    return {NAME = "", DESC = ""}
  end
  if seals[s] == nil then
    sendDebugMessage("Failed to read seal "..s)
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
    ["ENHANCEMENT"] = enhancement(c),
    ["SEAL"] = seal(c),
    ["DISABLED"] = c.debuff,
    ["FACE_DOWN"] = facing[c.facing],
  }
end

local function get_cards(a)
  local res = {}
  if a and a.cards then
    for _, c in ipairs(a.cards) do
      local set = c.ability.set
      if c.base and (set == "Default" or set == "Enhanced") then
        -- if c is a card
        res[#res + 1] = card(c)
      end
    end
  end
  return res
end

function get_deck()
  return get_cards(G.deck)
end

function get_hand()
  return get_cards(G.hand)
end

function get_shop_cards()
  return get_cards(G.shop_jokers)
end
