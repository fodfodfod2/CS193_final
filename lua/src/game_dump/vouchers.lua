assert(SMODS.load_file("description.lua"))()

function shop_voucher(v)
  local vars = card_vars(v)
  local key = v.config.center.key
  local name, desc = loc_text("Voucher", key, vars)
  return {NAME = name, DESC = desc}
end


function owned_voucher(v, key)
  local cfg = v.config or {}
  local vars = { cfg.extra, cfg.dollars, cfg.hand_size,
  cfg.joker_slot, cfg.hands, cfg.discards }
  local name, desc = loc_text("Voucher", key, vars)
  return {NAME = name, DESC = desc}
end

function get_owned_vouchers()
  local res = {}
  for key in pairs(G.GAME.used_vouchers or {}) do
    local center = G.P_CENTERS[key]
    if center then res[#res + 1] = owned_voucher(center, key) end
  end
  return res
end
