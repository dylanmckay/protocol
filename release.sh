#! /bin/sh -ea

DIR=$(dirname $0)

CURRENT_VERSION=$(sed -E 's/^version = "(.*)"/\1/;t;d' "$DIR/protocol/Cargo.toml")

cd protocol-derive
cargo publish

cd ../protocol
cargo publish

echo tagging $CURRENT_VERSION

git tag $CURRENT_VERSION
git push origin master $CURRENT_VERSION

echo "pls update the crate versions in the README if you changed the major or minor version"

