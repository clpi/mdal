#!/bin/sh
set -eu

CXX=clang++
OS=$(uname)
GIT_SHA=$(git rev-parse --short HEAD)
DISABLED_WARNINGS="-Wno-switch -Wno-macro-redefined -Wno-unused-value"

SRC="src/idx.cc"
OUT="dist/idx"
SRCD="src"
OUTD="dist"

CXXFLAGS=-"std=c++20 -DGIT_SHA=\"$GIT_SHA\""
LDFLAGS="-pthread -lm -lstdc++"
CXXFLAGS="$CXXFLAGS -DODIN_VERSION_RAW=\"dev-$(date +"%Y-%m")\""
XFLAGS=

panic() {
    printf "%s\n" "$1"
    exit 1
}

version() {
    version() { echo "$@" | awk -F. '{ printf("%d%03d%03d%03d\n", $1,$2,$3,$4); }'; }
}

darwin_cfg() {
    ARCH=$(uname -m)
    LLVM_CONFIG=llvm-config

    if [ ARCH == arm64 ]; then
        MIN_LLVM_VERSION=("13.0.0")
    else
        MIN_LLVM_VERSION=("11.1.0")
    fi

    if [ $(version $(LLVM_CONFIG --version)) -lt $(version $MIN_LLVM_VERSION) ]; then
        if [ ARCH == arm64 ]; then
            panic "LLVM must be > 13 for arm64"
        else
            panic "LLVM must be > 11 for arm64/x86"
        fi
    fi

    LDFLAGS="$LDFLAGS -liconv -ldl"
    CXXFLAGS="$CXXFLAGS $($LLVM_CONFIG --cxxflags --ldflags)"
    LDFLAGS="$LDFLAGS -lLLVM-C"
}

linux_cfg() {
    if which llvm-config > /dev/null 2>&1; then
        LLVM_CONFIG=llvm-config
    elif which llvm-config-11 > /dev/null 2>&1; then
        LLVM_CONFIG=llvm-config-11-64
    elif which llvm-config-11 > /dev/null 2>&1; then
        LLVM_CONFIG=llvm-config-11-64
    else
        panic "Can't find LLVM"
    fi
    MIN_LLVM_VERSION=("11.0.0")
    if [ $(version $($LLVM_CONFIG --version)) -lt $(version $MIN_LLVM_VERSION) ]; then
        echo "Tried to use " ${which $LLVM_CONFIG} "version $(LLVM_CONFIG- --version)"
        panic "LLVM version must be > 11"
    fi
    LDFLAGS="$LDFLAGS -ldl"
    CXXFLAGS="$CXXFLAGS $(LLVM_CONFIG --cxxflags --ldflags)"
    LDFLAGS="$LDFLAGS $(LLVM_CONFIG --libs core native --system-libs)"
}

build() {
    case $1 in
        debug | d ) XFLAGS="-g" ;;
        release | R ) XFLAGS="-O3" ;;
        release-native | N ) XFLAGS="-O3 -march=native" ;;
        dev | D ) XFLAGS="-DNIGHTLY -O3"
        *) panic "Build mode $1 unsupported"
    esac
    set -x
    $CXX $SRC $DISABLED_WARNINGS $CXXFLAGS $XFLAGS $LDFLAGS -c -o $OUT
    set -x
    
}

run() {
    case $OS in
        Linux ) linux_cfg ;;
        Darwin ) darwincfg ;;
        OpenBSD | FreeBSD ) ;;
        *) panic "Unspported" ;;
    esac
}

if [[ $# -eq 0 ]]; then
    build debug
    run debug
    exit 0
fi
