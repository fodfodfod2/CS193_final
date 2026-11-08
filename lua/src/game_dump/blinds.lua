assert(SMODS.load_file("tags.lua"))()

local function blind_amount(kind)
  local key = G.GAME.round_resets.blind_choices[kind]
  local b = G.P_BLINDS[key]
  if not b then return 0 end
  return math.floor(get_blind_amount(G.GAME.round_resets.blind_ante)
                    * b.mult * G.GAME.starting_params.ante_scaling)
end

local function blind_tag(kind)
  local key = G.GAME.round_resets.blind_tags and
              G.GAME.round_resets.blind_tags[kind]
  if not key then
    sendDebugMessage("failed to get tags for "..kind)
    return {}
  end

  local ok, t = pcall(Tag, key, true, kind)
  if not (ok and t) then return {} end
  return tag(t)
end

local function get_small_blind()
  local tag = blind_tag("Small")
  local score = blind_amount("Small")
  return {
    SCORE = score,
    TAG = tag,
  }
end

local function get_big_blind()
  local tag = blind_tag("Big")
  local score = blind_amount("Big")
  return {
    SCORE = score,
    TAG = tag,
  }
end

local function get_boss_blind()
  local score = blind_amount("Boss")
  local boss_key = G.GAME.round_resets.blind_choices.Boss
  local name, desc = loc_text("Blind", boss_key, {})
  return {
    SCORE = score,
    NAME = name,
    DESC = desc,
  }
end

function get_blinds()
  local nxt_blind = G.GAME.blind_on_deck
  cur = (nxt_blind == "Big" and 1) or (deck == "Boss" and 2) or 0

  return {
    SMALL = get_small_blind(),
    BIG = get_big_blind(),
    BOSS = get_boss_blind(),
    CURRENT = cur,
  }
end
