# Maintainer: Aldrin Zigmund Cortez Velasco <aldrinzigmund@tutamail.com>
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=zigfi-bin
pkgver=1.5.0
pkgrel=1
pkgdesc="zigfi is an open-source stocks, commodities and cryptocurrencies price monitoring CLI app, written fully in Rust, where you can organize assets you're watching easily into watchlists."
url="https://github.com/aldrinzigmundv/zigfi"
license=("AGPL-3.0-or-later")
arch=("x86_64")
provides=("zigfi")
conflicts=("zigfi")
source=("https://github.com/aldrinzigmundv/zigfi/releases/download/v$pkgver/zigfi-$pkgver-x86_64.tar.gz")
sha256sums=("ebc371d005832b6de8e90ca835b7aed929b2d0631c4ae126010090cfc014c915")

package() {
    install -Dm755 zigfi -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}