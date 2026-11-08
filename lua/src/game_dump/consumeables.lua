assert(SMODS.load_file("description.lua"))()



local function tarot(card)
    local name, desc = loc_text("Tarot", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.SELL_VALUE = card.sell_cost

    return res
end

local function planet(card)
    local name, desc = loc_text("Planet", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.HAND_ID = card.ability.hand_type
    res.SELL_VALUE = card.sell_cost

    return res
end

local function spectral(card)
    local name, desc = loc_text("Spectral", card.config.center.key, card_vars(card))
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

function get_last_played_planet_or_tarot()
  local key = G.GAME.last_tarot_planet
  local center = key and G.P_CENTERS[key]
  if not center then return false end

  local set = center.set

  local res = {
    TAROT = {},
    PLANET = {},
  }

  if set == "Tarot" then
    local name, desc = loc_text("Tarot", key, {})
    res.TAROT = {NAME = name, desc = DESC, SELL_VALUE = 0}
  elseif set == "Planet" then
    local name, desc = loc_text("Planet", key, {})
    res.PLANET = {
      NAME = name,
      desc = DESC, SELL_VALUE = 0, 
      HAND_ID = center.config.hand_type,
    }
  end

  return res
end
