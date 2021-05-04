# Maintainer: Devin Bidwell <dbidwell94@gmail.com>
pkgname=deep-search-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="An in-progress Linux terminal tool similar to Grep which aims to be faster and easer to use"
url="https://github.com/dbidwell94/deep-search"
license=("Apache-2.0")
arch=("x86_64")
provides=("deep-search")
source=("https://github.com/dbidwell94/deep-search/releases/download/v$pkgver/deep-search-$pkgver-x86_64.tar.gz")
sha256sums=("908da7786e92c39b8a414bfe6d84c8d03465d7bb2654b1963530591e68efca82")

package() {
    install -Dm755 deep-search -t "$pkgdir/usr/bin/"
}
