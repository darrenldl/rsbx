#!/bin/bash

TARGET=$HOME/kcov

if [[ $TRAVIS == true ]]; then
    export PATH=$TARGET/bin:$PATH
fi

function kcov_rsbx() {
    kcov --exclude-pattern=/.cargo,/usr/lib --verify "../target/cov/rsbx" rsbx "$@"
}
