from lib import RustDecks
from pathlib import Path

icons = list(Path("/usr/share/icons/gnome/48x48/emotes").glob("*.png"))
icons_raw = []
for i in range(15):
    with open(icons[i], 'rb') as icon_file:
        icons_raw.append( icon_file.read())

rust_decks = RustDecks(None)

for rust_deck in rust_decks.list_devices():
    print("Deck with serial: %s" % rust_deck.serial)

    for i in range(15):
        rust_deck.set_image(i, 'png', icons_raw[i])
