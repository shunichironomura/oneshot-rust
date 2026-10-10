#!/usr/bin/env nu
# Builds miri.wasm and its sysroot, and installs them into web/public/toolchain/.
#
# Steps:
#   1. Check out rust-lang/rust at RUST_COMMIT and apply patches/*.patch.
#   2. Download wasi-sdk.
#   3. Build Miri for the wasm32-wasip1 host (x.py, stage 2).
#   4. Build a native Miri + cargo-miri from the same tree and use it to build
#      the Miri sysroot (std with MIR) for the interpreted target.
#   5. Package miri.wasm + sysroot.tar under a content-addressed toolchain id.
#
# Everything is cached in toolchain/work/; rerunning only rebuilds what changed.

const RUST_COMMIT = "34f4a807e2d84184fff4e42d4539092569b6fcd6"
const WASI_SDK_VERSION = "32"
const INTERPRETED_TARGET = "x86_64-unknown-linux-gnu"

const TOOLCHAIN_DIR = path self .
const WORK_DIR = ($TOOLCHAIN_DIR | path join work)
const RUST_DIR = ($WORK_DIR | path join rust)
const SYSROOT_DIR = ($WORK_DIR | path join sysroot)
const PUBLIC_DIR = ($TOOLCHAIN_DIR | path join .. web public toolchain)

def build-triple []: nothing -> string {
  let arch = match $nu.os-info.arch { "aarch64" => "aarch64", "x86_64" => "x86_64", $a => { error make { msg: $"unsupported arch ($a)" } } }
  match $nu.os-info.name {
    "macos" => $"($arch)-apple-darwin"
    "linux" => $"($arch)-unknown-linux-gnu"
    $os => { error make { msg: $"unsupported OS ($os)" } }
  }
}

def wasi-sdk-dir []: nothing -> path {
  let arch = if $nu.os-info.arch == "aarch64" { "arm64" } else { "x86_64" }
  $WORK_DIR | path join $"wasi-sdk-($WASI_SDK_VERSION).0-($arch)-($nu.os-info.name)"
}

def ensure-wasi-sdk []: nothing -> path {
  let dir = wasi-sdk-dir
  if not ($dir | path exists) {
    let name = $dir | path basename
    let url = $"https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-($WASI_SDK_VERSION)/($name).tar.gz"
    print $"Downloading ($url)"
    http get $url | ^tar xzf - -C $WORK_DIR
  }
  $dir
}

def ensure-rust-checkout []: nothing -> nothing {
  if not ($RUST_DIR | path exists) {
    mkdir $RUST_DIR
    ^git -C $RUST_DIR init --quiet
    ^git -C $RUST_DIR remote add origin https://github.com/rust-lang/rust.git
  }
  let head = ^git -C $RUST_DIR rev-parse --verify --quiet HEAD | complete | get stdout | str trim
  if $head != $RUST_COMMIT {
    print $"Checking out rust-lang/rust ($RUST_COMMIT)"
    # Depth 2: bootstrap looks for the last merge commit to pick CI LLVM.
    ^git -C $RUST_DIR fetch --quiet --depth 2 origin $RUST_COMMIT
    ^git -C $RUST_DIR checkout --quiet --force FETCH_HEAD
  }
  for patch in (glob ($TOOLCHAIN_DIR | path join patches *.patch) | sort) {
    let name = $patch | path basename
    if (^git -C $RUST_DIR apply --check $patch | complete).exit_code == 0 {
      print $"Applying ($name)"
      ^git -C $RUST_DIR apply $patch
    } else if (^git -C $RUST_DIR apply --reverse --check $patch | complete).exit_code == 0 {
      print $"Already applied: ($name)"
    } else {
      error make { msg: $"($name) neither applies nor is applied; reset with `git -C ($RUST_DIR) checkout -- .`" }
    }
  }
}

def render-bootstrap-toml [wasi_sdk: path]: nothing -> nothing {
  open --raw ($TOOLCHAIN_DIR | path join bootstrap.toml)
  | str replace --all "@BUILD_TRIPLE@" (build-triple)
  | str replace --all "@WASI_SDK@" $wasi_sdk
  | str replace --all "@TOOLCHAIN_DIR@" $TOOLCHAIN_DIR
  | save --force ($RUST_DIR | path join bootstrap.toml)
}

def --wrapped x [wasi_sdk: path, ...args: string]: nothing -> nothing {
  cd $RUST_DIR
  with-env { WASI_SDK_PATH: $wasi_sdk } { ^./x.py ...$args }
}

def build-sysroot []: nothing -> nothing {
  let host = build-triple
  let build = $RUST_DIR | path join build $host
  let bin = $build | path join stage2 bin
  rm --recursive --force $SYSROOT_DIR
  let env_vars = {
    # The stage0 cargo is a beta release; the sysroot build needs `-Z` flags.
    RUSTC_BOOTSTRAP: "1"
    CARGO: ($build | path join stage0 bin cargo)
    MIRI: ($bin | path join miri)
    MIRI_LIB_SRC: ($RUST_DIR | path join library)
    MIRI_SYSROOT: $SYSROOT_DIR
    # The native Miri links against the stage-2 librustc_driver.
    DYLD_LIBRARY_PATH: ($build | path join stage2 lib)
    LD_LIBRARY_PATH: ($build | path join stage2 lib)
    # The dev sysroot has no rust-objcopy, which macOS needs to strip build scripts.
    CARGO_PROFILE_CUSTOM_SYSROOT_BUILD_OVERRIDE_STRIP: "false"
  }
  with-env $env_vars { ^($bin | path join cargo-miri) miri setup --target $INTERPRETED_TARGET }
}

def package []: nothing -> nothing {
  let staging = $WORK_DIR | path join out
  rm --recursive --force $staging
  mkdir $staging
  let miri_wasm = $staging | path join miri.wasm
  cp ($RUST_DIR | path join build wasm32-wasip1 stage2 bin miri.wasm) $miri_wasm
  # Miri only needs the rlibs (they carry the MIR); `.rmeta` and `.d` files are dropped,
  # as are crates that a player's binary crate cannot use.
  let unusable = ["test" "getopts" "proc_macro" "rustc_std_workspace_std"]
  let rlibs = glob ($SYSROOT_DIR | path join lib rustlib $INTERPRETED_TARGET lib *.rlib)
    | where { |p| ($p | path basename | parse --regex '^lib(?<name>.+)-[0-9a-f]+\.rlib$' | get 0.name) not-in $unusable }
    | each { path relative-to $SYSROOT_DIR }
    | sort
  let sysroot_tar = $staging | path join sysroot.tar
  with-env { COPYFILE_DISABLE: "1" } {
    ^tar --format ustar -cf $sysroot_tar -C $SYSROOT_DIR ...$rlibs
  }

  let id = [$miri_wasm $sysroot_tar] | each { open --raw $in | hash sha256 } | str join | hash sha256 | str substring 0..11
  let dest = $PUBLIC_DIR | path join $id
  rm --recursive --force $PUBLIC_DIR
  mkdir $dest
  mv $miri_wasm $sysroot_tar $dest
  {
    toolchainId: $id
    rustCommit: $RUST_COMMIT
    target: $INTERPRETED_TARGET
    files: {
      miriWasm: { path: $"($id)/miri.wasm", size: (ls ($dest | path join miri.wasm) | get 0.size | into int) }
      sysroot: { path: $"($id)/sysroot.tar", size: (ls ($dest | path join sysroot.tar) | get 0.size | into int) }
    }
  } | to json | save --force ($PUBLIC_DIR | path join manifest.json)
  print $"Installed toolchain ($id) into ($PUBLIC_DIR)"
}

def main []: nothing -> nothing {
  mkdir $WORK_DIR
  let wasi_sdk = ensure-wasi-sdk
  ensure-rust-checkout
  render-bootstrap-toml $wasi_sdk
  let host = build-triple
  x $wasi_sdk build --stage 2 miri --host wasm32-wasip1 --target wasm32-wasip1
  x $wasi_sdk build --stage 2 library src/tools/miri src/tools/miri/cargo-miri --host $host --target $host
  build-sysroot
  package
}
