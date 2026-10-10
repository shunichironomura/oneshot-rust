#!/bin/sh
# Linker wrapper for the wasm32-wasip1 host.
#
# Cargo puts the build compiler's `lib/` directory (containing `libLLVM.dylib`)
# on DYLD_LIBRARY_PATH / LD_LIBRARY_PATH. wasi-sdk's clang would then load that
# incompatible libLLVM instead of its own, so drop those variables.
# WASI_SDK_PATH must point to the extracted wasi-sdk.
unset DYLD_LIBRARY_PATH LD_LIBRARY_PATH
exec "$WASI_SDK_PATH/bin/clang" "$@"
