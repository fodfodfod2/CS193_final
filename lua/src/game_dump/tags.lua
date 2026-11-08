assert(SMODS.load_file("description.lua"))()

function tag(t)
  local name, desc = loc_text("Tag", t.key, card_vars(t))
  return {NAME = name ~= "" and name or tag.name, DESC = desc}
end

local function get_tags(a)
  local res = {}
  for _, j in ipairs(a or {}) do
    res[#res + 1] = tag(j)
  end
  return res
end


function get_owned_tags()
  if G.GAME.tags == nil then return {} end
  return get_tags(G.GAME.tags)
end
