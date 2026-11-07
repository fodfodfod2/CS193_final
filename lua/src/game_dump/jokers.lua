assert(SMODS.load_file("description.lua"))()

local editions = {
  ["foil"] = {NAME = "foil", DESC = "gain +50 chips when scored"},
  ["holo"] = {NAME = "holographic", DESC = "gain +10 mult when scored"},
  ["negative"] = {NAME = "negative", DESC = "does not take up a joker slot"},
  ["polychrome"] = {NAME = "polychrome", DESC = "*1.5 mult when scored"},
}


local facing = {
  ["front"] = false,
  ["back"] = true,
}


local function sticker(card)
  if card == nil then
    return {NAME = "", DESC = ""}
  end
  local k, v = "", ""
  if card.ability.eternal then
    k = "eternal"
    v = {}
  elseif card.ability.perishable then
    k = "perishable"
    v = {G.GAME.perishable_rounds, card.ability.perish_tally}
  elseif card.ability.rental then
    k = "rental"
    v = {G.GAME.rental_rate}
  else
    return {NAME = "", DESC = ""}
  end
  local name, desc = loc_text("Other",k,v)
  return {NAME = name, DESC = desc}
end

local function edition(card)
  if card.edition == nil then
    return {NAME = "", DESC = ""}
  end
  e = card.edition.type
  if e == nil then
    return {NAME = "", DESC = ""}
  end
  if editions[e] == nil then
    sendDebugMessage("failed to read edition"..tostring(e))
  end
  return editions[e]
end

function joker(j)
  if j == nil or j.base == nil then
    return nil
  end

  local invis = 0
  if j.ability.name == "Invisible Joker" then
    invis = math.max(0, (j.ability.invis_rounds or 0))
  end

  local perish = 0
  if j.ability.perishable then 
    perish = j.ability.perish_tally or 0 
  end

  local name, desc = loc_text("Joker", j.config.center.key, card_vars(j))
  return {
    ["NAME"] = name,
    ["DESC"] = desc,
    ["EDITION"] = edition(j),
    ["SELL_VALUE"] = j.sell_cost,
    ["DISABLED"] = j.debuff,
    ["STICKER"] = sticker(j),
    ["PERISH_CNT"] = perish,
    ["INVIS_ROUNDS"] = invis,
  }
end

local function get_jokers(a)
  local res = {}
  if a and a.cards then
    for i, j in ipairs(a.cards) do
      res[#res + 1] = joker(j)
    end
  end
  return res
end

function get_owned_jokers()
  return get_jokers(G.jokers)
end

function get_shop_jokers()
  return get_jokers(G.shop_jokers)
end
