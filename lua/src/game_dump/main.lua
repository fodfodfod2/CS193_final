local OUT_FILE = "DUMP.jkr"
local INTERVAL = 1
assert(SMODS.load_file("cards.lua"))()
assert(SMODS.load_file("jokers.lua"))()
assert(SMODS.load_file("vouchers.lua"))()

-- Credit for this func https://blog.stephansama.info/articles/how-to-print-a-lua-table/
local function dump(o)
  if type(o) == 'table' then
    local s = '{'
    ks = ""
    for k,v in pairs(o) do
      if type(k) ~= 'number' then ks = '"'..k..'"' else ks = k end
      s = s .. '['..ks..'] = ' .. dump(v) .. ','
    end
    return s .. '}'
  else
    return tostring(o)
  end
end

local function get_cards(a, fn)
  local res = {}
  if a and a.cards then
    for i, c in ipairs(a.cards) do
      res[#res + 1] = fn(c)
    end
  end
  return res
end


local function write_file(s)
  love.filesystem.write(OUT_FILE, s)
end

local function dump_data()
  local d = {}
  d.STATE = G.GAME_STATE
  d.DECK = get_cards(G.deck, card)
  d.VOUCHERS_OWNED = get_vouchers()
  d.JOKERS_OWNED = get_cards(G.jokers, joker)

  write_file(dump(d))
  sendDebugMessage("wrote playing_cards!")
  return true
end

local acc = 0
local orig_update = love.update
function love.update(dt)
  orig_update(dt)
  acc = acc + dt
  if acc >= (INTERVAL or 1) then
    acc = 0
    local ok, err = xpcall(dump_data, debug.traceback)
    if not ok then sendDebugMessage("dump_data FAILED: " .. tostring(err)) end
    sendDebugMessage("tick")
  end
end

