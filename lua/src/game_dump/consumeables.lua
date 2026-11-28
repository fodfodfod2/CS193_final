assert(SMODS.load_file("description.lua"))()


-- The constructor for a tarot card
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

-- The constructor for a planet card
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

-- The constructor for a spectral card
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

-- Get all of the consumeables in a given area of the given set using the given
-- constructor fn
function get_consumeable_from_area(area, set, fn)
  if area == nil then return nil end
  local cards = {}

  for i, c in ipairs(area.cards) do
    if c.ability.set == set then
      local card = fn(c)
      if card then
        cards[#cards+1] = fn(c)
        cards[#cards].IDX = i
      end
    end
  end
  return cards
end

-- Get all the planets from a given area
local function get_planets(area)
  return get_consumeable_from_area(area, "Planet", planet)
end

-- Get all the tarot cards from a given area
local function get_tarots(area)
  return get_consumeable_from_area(area, "Tarot", tarot)
end

-- Get all the spectral cards from a given area
local function get_spectrals(area)
  return get_consumeable_from_area(area, "Spectral", spectral)
end

-- Get all of the consumeables in the owned consumeables area
function get_owned_consumables()
  if G.consumeables == nil then return {} end
  if G.consumeables.cards == nil then return {} end
  local res = {}

  for i, c in ipairs(G.consumeables.cards) do
    if c.ability.set == "Planet" then
      local p = planet(c)
      if p then
        p.IDX = i
        res[#res+1] = p
      end
    elseif c.ability.set == "Tarot" then
      local t = tarot(c)
      if t then
        t.IDX = i
        res[#res+1] = t
      end
    elseif c.ability.set == "Spectral" then
      local s = spectral(c)
      if s then
        s.IDX = i
        res[#res+1] = s
      end
    end
  end
  return res
end

-- Get the planets from the shop
function get_shop_planets()
  return get_planets(G.shop_jokers)
end

-- Get the tarot cards from the shop
function get_shop_tarots()
  return get_tarots(G.shop_jokers)
end

-- Get the spectral cards from the shop
function get_shop_spectrals()
  return get_spectrals(G.shop_jokers)
end

function consumeable(c)
  if c.ability.set == "Planet" then
    return planet(c)
  elseif c.ability.set == "Tarot" then
    return tarot(c)
  elseif c.ability.set == "Spectral" then
    return spectral(c)
  end
end

-- Get the last played planet or tarot, this is used exclusively for the fool
-- tarot card
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
