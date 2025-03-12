# Maintainer: Ivan Chinenov <ichinenov@hjvt.dev>

pkgname=tstd
pkgver=0.1.0
pkgrel=1
pkgdesc="Command line utility for converting timestamps in various timezones and precisions to human-readable dates"
url="https://github.com/JohnDowson/$pkgname"
license=('MIT')
makedepends=('cargo')
depends=()
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
source=("$pkgname-$pkgver.tar.gz::$url/releases/download/v$pkgver/code.tar.gz")
sha512sum=({{SHA512}})

prepare() {
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    # for custom license, e.g. MIT
    # install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}