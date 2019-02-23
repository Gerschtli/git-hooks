with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "git-hooks";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  cargoSha256 = "1wy4yl5bcq3fr998garyjk7qz27l8j64d4bd5skfsrify9j9pf9c";
}
