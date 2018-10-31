from lib import RustDecks

from dbus.mainloop.glib import DBusGMainLoop

dbus_loop = DBusGMainLoop()

rust_decks = RustDecks(dbus_loop)

for rust_deck in rust_decks.list_devices():
    print("Deck with serial: %s" % rust_deck.serial)
    for i in range(5):
        rust_deck.fill_color(i, 51 * (i + 1), 0, 0)
    for i in range(5):
        rust_deck.fill_color(i + 5, 0, 51 * (i + 1), 0)
    for i in range(5):
        rust_deck.fill_color(i + 10, 0, 0, 51 * (i + 1))

def handle_key_up(serial, key):
    print("Do something fancy on up: %s %d" % (serial, key))

def handle_key_down(serial, key):
    print("Do something fancy on down: %s %d" % (serial, key))

rust_decks.on_key_up(handle_key_up)
rust_decks.on_key_down(handle_key_down)

from gi.repository import GLib

loop = GLib.MainLoop()
loop.run()