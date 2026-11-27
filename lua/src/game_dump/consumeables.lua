assert(SMODS.load_file("description.lua"))()



function tarot(card)
    local name, desc = loc_text("Tarot", card.config.center.key, card_vars(card))
    local res = {}
    res.TYPE = "TAROT"
    res.NAME = name
    res.DESC = desc
    res.COST = card.cost
    res.SELL_VALUE = card.sell_cost

    return res
end

function planet(card)
    local name, desc = loc_text("Planet", card.config.center.key, card_vars(card))
    local res = {}
    res.TYPE = "PLANET"
    res.NAME = name
    res.DESC = desc
    res.HAND_ID = card.ability.hand_type
    res.COST = card.cost
    res.SELL_VALUE = card.sell_cost

    return res
end

function spectral(card)
    local name, desc = loc_text("Spectral", card.config.center.key, card_vars(card))
    local res = {}
    res.TYPE = "SPECTRAL"
    res.NAME = name
    res.DESC = desc
    res.COST = card.cost
    res.SELL_VALUE = card.sell_cost

    return res
end

function get_consumeable_from_area(area, set, fn)
  if area == nil then return nil end
  cards = {}

  for i, c in ipairs(area.cards) do
    if c.ability.set == set then
      cards[#cards+1] = fn(c)
      cards[#cards].IDX = i
    end
  end
  return cards
end

local function get_planets(area)
  return get_consumeable_from_area(area, "Planet", planet)
end

local function get_tarots(area)
  return get_consumeable_from_area(area, "Tarot", tarot)
end

local function get_spectrals(area)
  return get_consumeable_from_area(area, "Spectral", spectral)
end

function get_owned_consumables()
  if G.consumeables == nil then return {} end
  if G.consumeables.cards == nil then return {} end
  local res = {}

  for _, c in ipairs(G.consumeables.cards) do
    if c.ability.set == "Planet" then
      res[#res+1] = planet(c)
    elseif c.ability.set == "Tarot" then
      res[#res+1] = tarot(c)
    elseif c.ability.set == "Spectral" then
      res[#res+1] = spectral(c)
    end
  end
  return res
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

function get_last_played_planet_or_tarot()
  local key = G.GAME.last_tarot_planet
  local center = key and G.P_CENTERS[key]
  if not center then return {TAROT={},PLANET={}} end

  local set = center.set

  local res = {
    TAROT = {},
    PLANET = {},
  }

  if set == "Tarot" then
    local name, desc = loc_text("Tarot", key, {})
    res.TAROT = {TYPE = "TAROT", NAME = name, DESC = desc, COST = 0, SELL_VALUE = 0}
  elseif set == "Planet" then
    local name, desc = loc_text("Planet", key, {})
    res.PLANET = {
      TYPE = "PLANET",
      NAME = name,
      DESC = desc, SELL_VALUE = 0,
      COST = 0,
      HAND_ID = center.config.hand_type,
    }
  end

  return res
end
