from lib import RustDecks
from pathlib import Path

icons = list(Path("/usr/share/icons/gnome/48x48/emotes").glob("*.png"))
icons_raw = []
for icon in icons:
    with open(icon, 'rb') as icon_file:
        icons_raw.append( icon_file.read())

rust_decks = RustDecks(None)

image_counter = 0
for rust_deck in rust_decks.list_devices():
    print("Deck with serial: %s" % rust_deck.serial)

    for i in range(15):
        rust_deck.set_image(i, 'png', icons_raw[image_counter])
        image_counter += 1
