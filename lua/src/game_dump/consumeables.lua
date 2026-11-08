assert(SMODS.load_file("description.lua"))()



local function tarot(card)
    local name, desc = loc_text("tarot", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.SELL_VALUE = card.sell_cost

    return res
end

local function planet(card)
    local name, desc = loc_text("planet", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.HAND_ID = card.ability.hand_type
    res.SELL_VALUE = card.sell_cost

    return res
end

local function spectral(card)
    local name, desc = loc_text("spectral", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.SELL_VALUE = card.sell_cost

    return res
end

local function get_consumeable_from_area(area, set, fn)
  if area == nil then return nil end
  cards = {}

  for _, c in ipairs(area.cards) do
    if c.ability.set == set then
      cards[#cards+1] = fn(c)
    end
  end
  return cards
end

local function get_planets(area)
  return get_consumeable_from_area(area, "planet", planet)
end

local function get_tarots(area)
  return get_consumeable_from_area(area, "tarot", tarot)
end

local function get_spectrals(area)
  return get_consumeable_from_area(area, "spectral", spectral)
end

function get_owned_consumables()
  if G.consumeables == nil then return {} end
  if G.consumeables.cards == nil then return {} end
  return {
    PLANET = get_planets(G.consumeables),
    TAROT = get_tarots(G.consumeables),
    SPECTRAL = get_spectrals(G.consumeables),
  }
end

function get_shop_planets()
  return get_planets(G.shop_jokers)
end

function get_shop_tarots()
  return get_tarots(G.shop_jokers)
end

function get_shop_spectrals()
  return get_spectrals(G.shop_jokers)
end
