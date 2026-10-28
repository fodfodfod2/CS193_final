local OUT_FILE = "DUMP.jkr"
local INTERVAL = 1.0
assert(SMODS.load_file("cards.lua"))()
assert(SMODS.load_file("jokers.lua"))()

-- Credit for this func https://blog.stephansama.info/articles/how-to-print-a-lua-table/
local function dump(o)
   if type(o) == 'table' then
      local s = '{ '
      ks = ""
      for k,v in pairs(o) do
         if type(k) ~= 'number' then ks = '"'..k..'"' else ks = k end
         s = s .. '['..ks..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
end

local function write_file(s)
  local path = SMODS.current_mod.path .. OUT_FILE
  local file = io.open(path, "w")
  if file then
      file:write(s)
      file:close()
      sendDebugMessage("Successfully wrote to " .. path)
  else
    sendDebugMessage("Failed to write to " .. path)
  end

end

function main()
  if not G.playing_cards == nil then
    for p in G.playing_cards do
      write_file(dump(card(p)))
    end
    sendDebugMessage("wrote playing_cards!")
  else
    sendDebugMessage("playing_cards is nil, moving on")
  end
  G.E_MANAGER:add_event(Event{
    trigger = "after",
    delay = INTERVAL,
    front = true,
    func = function ()
            main()
           end}, "dump_queue")
end

G.E_MANAGER.queues.dump_queue = {}
main()
