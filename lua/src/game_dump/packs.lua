assert(SMODS.load_file("cards.lua"))()
assert(SMODS.load_file("jokers.lua"))()
assert(SMODS.load_file("consumeables.lua"))()

-- This is the constructor for a pack in the shop before it is bought
function shop_pack(p)
  local center = p.config.center
  local cfg = center.config or {}
  local name, _ = loc_text("Other", center.key, { cfg.choose, cfg.extra })

  local type = ""
  for word in p.ability.name:gmatch("%S+") do
    if word == "Standard" or word == "Buffoon" or word == "Celestial"
      or word == "Spectral" or word == "Arcana" then
      type = word
      break
    end
  end
  if type == "" then
    sendDebugMessage("Pack type conversion failed, name is \""..name.."\"");
  end
  return {
    NAME = name ~= "" and name or p.ability.name,
    TYPE = type,
    SELECT_NUM = cfg.choose or 1,
    SIZE = cfg.extra or 0,
    COST = p.cost,
  }
end

-- This is the constructor to get an opened pack during pack openings
function get_open_pack()
  if not (G.pack_cards and G.pack_cards.cards) then return nil end

  local res = {}
  res.CONTENTS = {}
  local fn
  local cards = G.pack_cards.cards
  res.SELECT_NUM = G.GAME.pack_choices
  if cards[1].ability.set == "Tarot" or
    cards[2].ability.set == "Tarot" then
    res.TYPE = "Tarot"
    fn = consumeable
  elseif cards[1].ability.set == "Planet" then
    res.TYPE = "Planet"
    fn = consumeable
  elseif cards[1].ability.set == "Spectral" then
    res.TYPE = "Spectral"
    fn = consumeable
  elseif cards[1].ability.set == "Joker" then
    res.TYPE = "Joker"
    fn = joker
  elseif cards[1].ability.set == "Default" or cards[1].ability.set == "Enhanced" then
    res.TYPE = "Card"
    fn = card
  else
    sendDebugMessage("the type is "..cards[1].ability.set..".")
  end

  for i, c in ipairs(cards) do
    local parse_res = fn(c)
    if parse_res then
      res.CONTENTS[i] = parse_res
      res.CONTENTS[i].IDX = i
    end
  end
  return res
end

-- Get the packs from the shop
function get_shop_packs()
  if not (G.shop_booster and G.shop_booster.cards) then return nil end

  local res = {}
  for i,c in ipairs(G.shop_booster.cards) do
    res[i] = shop_pack(c)
  end
  return res
end
