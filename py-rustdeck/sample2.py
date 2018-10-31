from lib import RustDecks
from pathlib import Path

rust_decks = RustDecks(None)

for rust_deck in rust_decks.list_devices():
    print("Deck with serial: %s" % rust_deck.serial)
    icons = list(Path("/usr/share/icons/gnome/48x48/emotes").glob("*.png"))
    for i in range(15):
        with open(icons[i], 'rb') as content_file:
            content = content_file.read()
        rust_deck.set_image(i, 'png', content)
