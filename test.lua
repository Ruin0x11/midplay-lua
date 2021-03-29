package.cpath = package.cpath .. ";./target/release/?.dll;" .. ";./target/debug/?.dll;"
local midplay = require("midplay_lua")

local ports = midplay.generic.get_ports()
print("Available ports:")
for _, port in ipairs(ports) do
   print(port.index, port.name)
end

print()

print("Playing!")
midplay.generic.play_midi("data/tym00.mid", ports[1].index)
print("Is playing:", midplay.generic.is_midi_playing())

io.flush()
io.read()

print("Stopping!")
midplay.generic.stop_midi()
print("Is playing:", midplay.generic.is_midi_playing())

io.flush()
io.read()

print("Playing! (native)")
midplay.native.play_midi("data/tym00.mid")
print("Is playing:", midplay.native.is_midi_playing())

io.flush()
io.read()

print("Stopping! (native)")
midplay.native.stop_midi()
print("Is playing:", midplay.native.is_midi_playing())
