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


local function sticker(s)
  if s == nil then
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

local function edition(e)
  if e == nil then
    return {NAME = "", DESC = ""}
  end
  sendDebugMessage("got editions:"..e)
  return editions[e]
end

function joker(j)
  if c == nil or j.base == nil then
    return nil
  end
  local name, desc = loc_text("Joker", j.config.center.key, card_vars(j))
  return {
    ["NAME"] = name,
    ["DESC"] = desc,
    ["EDITION"] = edition(j.edition),
    ["SELL_VALUE"] = j.sell_cost,
    ["DISABLED"] = j.debuff,
    ["STICKER"] = sticker(j.sticker),
    ["PERISH_CNT"] = j.perish_tally,
    ["INVIS_ROUNDS"] = j.invis_rounds,
  }
end
