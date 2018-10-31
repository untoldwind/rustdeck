import dbus
from .device import RustDeck

class RustDecks:
    def __init__(self, mainloop):
        self._system_bus = dbus.SystemBus(mainloop=mainloop)
        devices = self._system_bus.get_object('io.github.rustdeck1', '/devices')
        self._devices = dbus.Interface(devices, dbus_interface='io.github.rustdeck1.Devices')

    def list_devices(self):
        devices = []

        for device_path in self._devices.ListDevices():
            device = self._system_bus.get_object('io.github.rustdeck1', device_path)
            devices.append(RustDeck(device))

        return devices

    def on_key_up(self, handler):
        self._devices.connect_to_signal('key_up', handler)

    def on_key_down(self, handler):
        self._devices.connect_to_signal('key_down', handler)
