#!/usr/bin/env nu
# Uploads the toolchain in web/public/toolchain/ (built by build.nu) as a GitHub
# Release and pins it in toolchain/RELEASE, so that CI and other developers can
# fetch it with `web/scripts/fetch-toolchain.ts` instead of building it (20-30 minutes).

const TOOLCHAIN_DIR = path self .
const PUBLIC_DIR = ($TOOLCHAIN_DIR | path join .. web public toolchain)
const RELEASE_FILE = ($TOOLCHAIN_DIR | path join RELEASE)

def main []: nothing -> nothing {
  let manifest_path = $PUBLIC_DIR | path join manifest.json
  let manifest = open $manifest_path
  let id = $manifest.toolchainId
  let tag = $"toolchain-($id)"
  let parts = $manifest.files | values | each { get parts } | flatten | each { |p| $PUBLIC_DIR | path join $p }
  let licenses = $manifest.licenses | each { |p| $PUBLIC_DIR | path join $p }

  if (^gh release view $tag | complete).exit_code == 0 {
    # The assets are the same, but the manifest and the licenses may be newer (a release made
    # before the licenses were collected gets them this way).
    print $"Release ($tag) already exists; updating its manifest and licenses"
    ^gh release upload $tag $manifest_path ...$licenses --clobber
  } else {
    let notes = $"miri.wasm and its sysroot, built by toolchain/build.nu from rust-lang/rust@($manifest.rustCommit).\n\nThe license texts of their components are in the assets: start with README.txt.\n\nFetch with `node web/scripts/fetch-toolchain.ts`."
    ^gh release create $tag $manifest_path ...$parts ...$licenses --title $"Toolchain ($id)" --notes $notes --latest=false
  }
  $"($tag)\n" | save --force $RELEASE_FILE
  print $"Pinned ($tag) in ($RELEASE_FILE); commit it."
}
