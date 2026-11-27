assert(SMODS.load_file("description.lua"))()

-- This is the constructor for a voucher still in the shop
function shop_voucher(v)
  local vars = card_vars(v)
  local key = v.config.center.key
  local name, desc = loc_text("Voucher", key, vars)
  return {NAME = name, COST = v.cost, DESC = desc}
end

-- This is the constructor for a voucher owned by the player
local function owned_voucher(v, key)
  local cfg = v.config or {}
  local vars = { cfg.extra, cfg.dollars, cfg.hand_size,
  cfg.joker_slot, cfg.hands, cfg.discards }
  local name, desc = loc_text("Voucher", key, vars)
  return {NAME = name, COST = 0, DESC = desc}
end

-- Get the owned vouchers
function get_owned_vouchers()
  local res = {}
  for key in pairs(G.GAME.used_vouchers or {}) do
    local center = G.P_CENTERS[key]
    if center then res[#res + 1] = owned_voucher(center, key) end
  end
  return res
end

-- Get the vouchers from the shop
function get_shop_vouchers()
  local res = {}
  local cards = G.shop_vouchers and G.shop_vouchers.cards or {}
  for _, card in ipairs(cards) do
    res[#res + 1] = shop_voucher(card)
  end
  return res
end
