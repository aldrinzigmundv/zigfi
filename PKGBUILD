# Maintainer: Aldrin Zigmund Cortez Velasco <aldrinzigmund@tutamail.com>
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=zigfi-bin
pkgver=2.0.0
pkgrel=1
pkgdesc="zigfi is an open-source stocks, commodities and cryptocurrencies price monitoring CLI app, written fully in Rust, where you can organize assets you're watching easily into watchlists."
url="https://github.com/aldrinzigmundv/zigfi"
license=("AGPL-3.0-or-later")
arch=("x86_64")
provides=("zigfi")
conflicts=("zigfi")
source=("https://github.com/aldrinzigmundv/zigfi/releases/download/v$pkgver/zigfi-$pkgver-x86_64.tar.gz")
sha256sums=("438f2e30e2dd2422b6965ac437448eed72bf23f6bb326f46ca511473f388aaa6")

package() {
    install -Dm755 zigfi -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
