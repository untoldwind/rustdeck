import dbus

class RustDeck:
    def __init__(self, device):
        self._device = dbus.Interface(device, dbus_interface='io.github.rustdeck1.Device')

    @property
    def serial(self):
        return self._device.GetSerial()

    def fill_color(self, key, red, green, blue):
        self._device.FillColor(key, red, green, blue)

    def set_image(self, key, format, data):
        self._device.SetImage(key, format, data)
