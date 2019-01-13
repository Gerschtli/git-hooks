with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "git-hooks";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  cargoSha256 = "09wv4jvyh077a56zn5ihdl0psqccz3xqh1w769byddzaggirqgij";
}
