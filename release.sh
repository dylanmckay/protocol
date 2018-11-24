#! /bin/sh -ea

DIR=$(dirname $0)

CURRENT_VERSION=$(sed -E 's/^version = "(.*)"/\1/;t;d' "$DIR/protocol/Cargo.toml")

if ! git diff-index --quiet HEAD --; then
  echo "git worktree has uncomitted changes" 1>&2
  exit 1
fi

function update_manifest_version {
  new_version=$1
  find -name Cargo.toml -not -path "*/target/*" -exec sed -Ei "s/$CURRENT_VERSION/$new_version/g" "{}" \+
  git commit -am "Bump version to $new_version"
}

if [ -z "$1" ]; then echo "please pass a new version number on the cmdline" && exit 1; fi
update_manifest_version $1

cd protocol-derive
cargo publish

cd ../protocol
cargo publish

echo tagging $CURRENT_VERSION

git tag $CURRENT_VERSION
git push origin master $CURRENT_VERSION

echo "pls update the crate versions in the README if you changed the major or minor version"

