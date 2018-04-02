# This script takes care of testing your crate

set -ex

main() {
    cat > Cross.toml <<EOF
[target.i686-unknown-linux-gnu]
image = "cryze/i686-unknown-linux-gnu-wasmboy"
[target.i686-unknown-linux-musl]
image = "cryze/i686-unknown-linux-musl-wasmboy"
[target.x86_64-unknown-linux-gnu]
image = "cryze/x86_64-unknown-linux-gnu-wasmboy"
[target.x86_64-unknown-linux-musl]
image = "cryze/x86_64-unknown-linux-musl-wasmboy"
EOF

    cross build --target $TARGET
    cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
    cross test --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
