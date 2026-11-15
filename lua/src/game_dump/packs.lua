assert(SMODS.load_file("cards.lua"))()
assert(SMODS.load_file("jokers.lua"))()
assert(SMODS.load_file("consumeables.lua"))()

function shop_pack(p)
  local center = p.config.center
  local cfg = center.config or {}
  local name, _ = loc_text("Other", center.key, { cfg.choose, cfg.extra })

  local type = ""
  for word in p.ability.name:gmatch("%S+") do
    sendDebugMessage("\""..word.."\"")
    if word == "Standard" or word == "Buffoon" or word == "Celestial"
       or word == "Spectral" or word == "Arcana" then
      type = word
      break
    end
  end
  if type == "" then
    sendDebugMessage("Pack type conversion failed, name is "..name);
  end
  return {
    NAME = name ~= "" and name or p.ability.name,
    TYPE = type,
    SELECT_NUM = cfg.choose or 1,
    SIZE = cfg.extra or 0,
    COST = p.cost,
  }
end

function get_open_pack()
  if not (G.pack_cards and G.pack_cards.cards) then return nil end

  local res = {}
  local cards = G.pack_cards.cards
  res.SELECT_NUM = G.GAME.pack_choices
  if cards[1].ability.set == "Tarot" then
    res.TYPE = "Tarot"
    res.CONTENTS = {}
    for i,c in ipairs(cards) do
      res.CONTENTS[i] = tarot(c)
    end
  elseif cards[1].ability.set == "Planet" then
    res.TYPE = "Planet"
    res.CONTENTS = {}
    for i,c in ipairs(cards) do
      res.CONTENTS[i] = planet(c)
    end
  elseif cards[1].ability.set == "Spectral" then
    res.TYPE = "Spectral"
    res.CONTENTS = {}
    for i,c in ipairs(cards) do
      res.CONTENTS[i] = spectral(c)
    end
  elseif cards[1].ability.set == "Joker" then
    res.TYPE = "Joker"
    res.CONTENTS = {}
    for i,c in ipairs(cards) do
      res.CONTENTS[i] = joker(c)
    end
  else
    sendDebugMessage("the type is "..cards[1].ability.set..".")
  end

  return res
end

function get_shop_packs()
  if not (G.shop_booster and G.shop_booster.cards) then return nil end

  local res = {}
  for i,c in ipairs(G.shop_booster.cards) do
    res[i] = shop_pack(c)
  end
  return res
end
