with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "git-hooks";

  src = builtins.filterSource
    (path: type: type != "directory" || baseNameOf path != "target")
    ./.;

  cargoSha256 = "18qdsdsl0df6y93z1mmnnn6npcg4i1aizsmyxqfif45vc55wrql8";
}
