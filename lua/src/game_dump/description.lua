-- Get the variables from a card area, this is used for fetching names and
-- descriptions for pretty much every item. These are then used to fill in the
-- description of a given object
function card_vars(card)
    local ok, vars = pcall(card.generate_UIBox_ability_table, card, true)
    if ok and type(vars) == "table" then return vars end
    return {}
end

-- This will get the description of a given item
-- set is the set the item is from eg: "Tarot"
-- key is the actual key for the item
-- vars is the variabls at runtime needed to fill in the blanks,
-- get them from card_vars()
function loc_text(set, key, vars)
    local sets = G.localization and G.localization.descriptions
    local entry = sets and sets[set] and sets[set][key]
    if not entry then return "", "" end
    local lines = {}
    for _, line in ipairs(entry.text or {}) do
        line = line:gsub("#(%d+)#", function(n)
            local v = vars and vars[tonumber(n)]
            if type(v) == "number" or type(v) == "string" then return tostring(v) end
            return ""
        end)
        line = line:gsub("%b{}", "")
        lines[#lines + 1] = line
    end
    return entry.name or "", table.concat(lines, " ")
end


