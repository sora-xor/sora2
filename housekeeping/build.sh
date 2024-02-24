#!/bin/bash -x
set -e

# environment
palletListFile='pallet_list.txt'
wasmReportFile='subwasm_report.json'
PACKAGE='framenode-runtime'
RUSTFLAGS='-Dwarnings'
RUNTIME_DIR='runtime'
allfeatures='private-net,wip,ready-to-test'

# build func
test() {
    echo buildTag is $buildTag
    echo TagName is ${TAG_NAME}
    if [[ $buildTag != null ]] && [[ ${TAG_NAME} != null || ${TAG_NAME} != '' ]]; then
        printf "⚡️ Testing with features: private-net runtime-benchmarks\n"
        cargo test --release --features "private-net runtime-benchmarks"
    elif [[ $prBranch = 'master' ]]; then
        printf "⚡️ This is "$prBranch" Running tests and migrations %s\n"
        RUST_LOG="debug cargo test --features try-runtime -- run_migrations"
    else
        printf "⚡️ Running Tests for code coverage only\n"
        export RUSTFLAGS="-Cinstrument-coverage"
        export SKIP_WASM_BUILD=1
        export LLVM_PROFILE_FILE="sora2-%p-%m.profraw"
        rm -rf ~/.cargo/.package-cache
        cargo fmt -- --check > /dev/null
        cargo test --features $allfeatures
    fi
}

build() {
    printf "Tag is %s\n" ${TAG_NAME}
    printf "BuildTag is %s\n" $buildTag
    sudoCheckStatus="0"
    if [[ ${TAG_NAME} =~ 'benchmarking'* ]]; then
        featureList='private-net runtime-benchmarks'
    elif [[ ${TAG_NAME} =~ 'stage'* ]]; then
        featureList='private-net include-real-files ready-to-test'
    elif [[ ${TAG_NAME} =~ 'test'* ]]; then
        featureList='private-net include-real-files reduced-pswap-reward-periods ready-to-test'
    elif [[ -n ${TAG_NAME} && ${TAG_NAME} != 'predev' ]]; then
        featureList='include-real-files'
        sudoCheckStatus="101"
    elif [[ -n $buildTag ]]; then
        featureList='private-net include-real-files reduced-pswap-reward-periods wip ready-to-test'
    fi
    printf "⚡️ Building with features: %s\n" "$featureList"
    printf "⚡️ Checking sudo pallet: %s\n" "$sudoCheckStatus"
    rm -rf target
    cargo build --release --features "$featureList"
    mv ./target/release/framenode .
    mv ./target/release/wbuild/framenode-runtime/framenode_runtime.compact.compressed.wasm ./framenode_runtime.compact.compressed.wasm
    subwasm --json info framenode_runtime.compact.compressed.wasm > $wasmReportFile
    subwasm metadata framenode_runtime.compact.compressed.wasm > $palletListFile
    set +e
    subwasm metadata -m Sudo framenode_runtime.compact.compressed.wasm
    if [[ $? -eq $sudoCheckStatus ]]; then 
        echo "✅ sudo check is successful!"
    else 
        echo "❌ sudo check is failed!"
        exit 1
    fi
}

if [ "$(type -t $1)" = "function" ]; then
    "$1"
else
    echo "Func '$1' is not exists in this workflow. Skipped."
fi
