# This script takes care of testing your crate

set -ex

main() {
    xargo check --target $TARGET
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
