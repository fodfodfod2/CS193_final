assert(SMODS.load_file("description.lua"))()

local function tag(t)
  local name, desc = loc_text("Tag", t.key, card_vars(t))
  return {NAME = name ~= "" and name or tag.name, DESC = desc}
end

local function get_tags(a)
  local res = {}
  if a and a.cards then
    for i, j in ipairs(a.cards) do
      res[#res + 1] = tag(j)
    end
  end
  return res
end


function get_owned_tags()
  if G.GAME.tags == nil then return {} end
  get_tags(G.GAME.tags)
end
