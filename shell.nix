with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "git-hooks";

  buildInputs = [
    cargo-edit
    rustup
  ];
}
