PKG_NAME=$1
registry_version=`cargo search $PKG_NAME | awk '{print $3}' | tr -d \"`
local_version=`cargo pkgid -p $PKG_NAME | cut -d'#' -f2`

echo "registry_version: $registry_version"
echo "local_version: $local_version"

if [ "$registry_version" = "$local_version" ]; then
    echo "No need to publish"
    exit 0
fi

echo "Publishing $PKG_NAME"
cargo publish -p $PKG_NAME

