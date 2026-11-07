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
      s = s .. '['..ks..']=' .. dump(v) .. ','
    end
    return s .. '}'
  else
    if type(o) == 'string' then return '"'..o..'"' end
    return tostring(o)
  end
end




local function write_file(s)
  love.filesystem.write(OUT_FILE, s)
end

local function dump_data()
  local d = {}
  d.STATE = G.STATE
  d.DECK = get_deck()
  d.VOUCHERS_OWNED = get_owned_vouchers()
  d.JOKERS_OWNED = get_owned_jokers()
  d.HAND = get_hand()

  local ctx = {}
  ctx.HANDS = G.GAME.current_round.hands_left
  ctx.DISCARDS = G.GAME.current_round.discards_left
  ctx.MAX_HANDS = G.GAME.round_resets.hands
  ctx.MAX_DISCARDS = G.GAME.round_resets.discards

  local hand_lvls = {}
  local hands_played = {}
  local max_played = 0
  local max_played_hand = ""
  for i, name in ipairs(G.handlist) do
    local h = G.GAME.hands[name]
    hand_lvls[name] = h.level
    hands_played[name] = (h.played_this_round and 1) or 0
    if h.played > max_played then
      max_played = h.played
      max_played_hand = name
    end
  end
  ctx.HANDS_PLAYED = hands_played
  ctx.MOST_PLAYED_HAND = max_played_hand
  ctx.HAND_LVLS = hand_lvls

  d.CTX = ctx
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

