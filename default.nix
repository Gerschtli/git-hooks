with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "git-hooks";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  cargoSha256 = "0dymyzghxhkyhjr4n99sb9nvcws7pbx48hhis2rv40cyr52k1k24";
}
