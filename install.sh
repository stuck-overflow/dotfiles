#!/bin/bash -e
#
# This script compiles and install the tmux widget. By default those are
# installed in ~/bin. Alternatively you can pass the destination directory as
# argument.
destdir=${1:-~/bin}
widgets="tmux-widget pomodoro-widget"

if [ \! -d "$destdir" ]
then
    echo "Destination directory ${destdir} doesn't exist, exiting."
    exit 1
fi

if ! command -v cargo &> /dev/null
then
    echo "cargo not found"
    exit 1
fi

curdir=${PWD}

for widget in "${widgets}"; do
    cd ${curdir}/${widget}
    cargo clean
    cargo update
    cargo build --release
    cp -v target/release/${widget} ~/bin
done
