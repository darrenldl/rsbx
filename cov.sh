#!/bin/bash

if [[ $TRAVIS == true ]]; then
    if ! [[ $TARGET == x86_64-unknown-linux-gnu && $DISABLE_COV == "" ]]; then
        exit 0
    fi
fi

if [[ $TRAVIS == true ]]; then
    export PATH=$HOME/kcov/bin:$PATH
fi

# export RUSTFLAGS="-C link-dead-code"

echo "Running cargo tests"
echo "========================================"
rm -rf target/debug/
cargo build --tests
if [[ $? != 0 ]]; then
    exit 1
fi

COV_DIR="target/cov/cargo-tests"
for file in target/debug/blkar_lib-*; do
    if [[ $file == *.d ]]; then
        continue
    fi

    mkdir -p $COV_DIR
    kcov --exclude-pattern=/.cargo,/usr/lib --verify $COV_DIR "$file"
done

echo ""
echo "Running binary tests"
echo "========================================"
cd cov_tests/
./dev_tests.sh
if [[ $? != 0 ]]; then
    exit 1
fi
cd ..

echo ""
echo "========================================"
echo ""
echo "Merging all code coverage reports"
echo ""
echo "========================================"
rm -rf target/cov/total
mkdir -p target/cov/total
kcov --merge target/cov/total $COV_DIR target/cov/bin-tests
