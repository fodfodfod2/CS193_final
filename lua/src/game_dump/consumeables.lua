assert(SMODS.load_file("description.lua"))()



function tarot(card)
    local name, desc = loc_text("tarot", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.SELL_VALUE = card.sell_cost

    return res
end

function planet(card)
    local name, desc = loc_text("planet", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.HAND_ID = card.ability.hand_type
    res.SELL_VALUE = card.sell_cost

    return res
end

function spectral(card)
    local name, desc = loc_text("spectral", card.config.center.key, card_vars(card))
    local res = {}
    res.NAME = name
    res.DESC = desc
    res.SELL_VALUE = card.sell_cost

    return res
end
