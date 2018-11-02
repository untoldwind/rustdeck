pkgname=rustdeck
pkgver=0.1
pkgrel=5
pkgdesc="Driver and tools for Elgato StreamDeck"
arch=('x86_64')
url="https://github.com/untoldwind/rustdeck"
makedepends=('rustup')

build() {
    rustup run stable cargo build --release
}

package() {
    mkdir -p "$pkgdir/usr/bin"
    cp "$PWD/../target/release/rustdeck-daemon" "$pkgdir/usr/bin/rustdeck-daemon"
    mkdir -p "$pkgdir/usr/share/dbus-1/system.d"
    cp "$PWD/../daemon/io.github.rustdeck1.conf" "$pkgdir/usr/share/dbus-1/system.d/io.github.rustdeck1.conf"
    mkdir -p "$pkgdir/usr/share/dbus-1/system-services"
    cp "$PWD/../daemon/io.github.rustdeck1.service" "$pkgdir/usr/share/dbus-1/system-services/io.github.rustdeck1.service"
    mkdir -p "$pkgdir/usr/lib/systemd/system"
    cp "$PWD/../daemon/rustdeck-daemon.service" "$pkgdir/usr/lib/systemd/system/rustdeck-daemon.service"
}
