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
sha256sums=("0ed35817a2f9d7358293ba9460dc0e02ac67a24a29e5e9807fe4440c2b97f696")

package() {
    install -Dm755 deep-search -t "$pkgdir/usr/bin/"
}
