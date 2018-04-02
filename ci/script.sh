# This script takes care of testing your crate

set -ex

main() {
    PKG_CONFIG_ALLOW_CROSS=1 cross build --target $TARGET
    PKG_CONFIG_ALLOW_CROSS=1 cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    PKG_CONFIG_ALLOW_CROSS=1 cross test --target $TARGET
    PKG_CONFIG_ALLOW_CROSS=1 cross test --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
