#! /bin/sh -ea

DIR=$(dirname $0)

NEW_VERSION=$1
if [ -z "$1" ]; then echo "please pass a new version number on the cmdline" && exit 1; fi

PREVIOUS_VERSION=$(sed -E 's/^version = "(.*)"/\1/;t;d' "$DIR/protocol/Cargo.toml")

if ! git diff-index --quiet HEAD --; then
  echo "git worktree has uncomitted changes" 1>&2
  exit 1
fi

function update_manifest_version {
  find -name Cargo.toml -not -path "*/target/*" -exec sed -Ei "s/$PREVIOUS_VERSION/$NEW_VERSION/g" "{}" \+
  git commit -am "Bump version to $NEW_VERSION"
}

update_manifest_version $1

cd protocol-derive
cargo publish

cd ../protocol
cargo publish

echo tagging $NEW_VERSION

git tag $NEW_VERSION
git push origin master $NEW_VERSION

echo "pls update the crate versions in the README if you changed the major or minor version"

