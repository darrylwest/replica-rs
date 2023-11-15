#!/usr/bin/env bash
# dpw@plaza.localdomain
# 2023-11-15 20:43:49
#

set -eu

os=`uname`

if [ $os == "Darwin" ]
then
    mv ./target/release/replica /usr/local/bin/replica
else
    sudo mv ./target/release/replica /usr/local/bin/replica
fi

echo "ok."
replica --version


