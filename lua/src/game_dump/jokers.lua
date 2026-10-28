
local editions = {
  ["foil"] = {"foil", "gain +50 chips when scored"},
  ["holo"] = {"holographic", "gain +10 mult when scored"},
  ["negative"] = {"negative", "does not take up a joker slot"},
  ["polychrome"] = {"polychrome", "*1.5 mult when scored"},
}

local facing = {
  ["front"] = false,
  ["back"] = true,
}


local function sticker(s)
  if s == nil then
    return {"",""}
  end
  sendDebugMessage("got stickers:"..s)
  return stickers[s]
end

local function edition(e)
  if e == nil then
    return {"",""}
  end
  sendDebugMessage("got editions:"..e)
  return editions[e]
end

function joker(j)
  if c == nil or j.base == nil then
    return nil
  end
  return {
    ["NAME"] = j.name,
    ["DESC"] = j.effect,
    ["EDITION"] = edition(j.edition),
    ["SELL_VALUE"] = j.sell_cost,
    ["DISABLED"] = j.debuff,
    ["STICKER"] = sticker(j.sticker),
    ["PERISH_CNT"] = j.perish_tally,
    ["INVIS_ROUNDS"] = j.invis_rounds,
  }
end
