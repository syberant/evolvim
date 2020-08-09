{ pkgs ? import <nixpkgs> { } }:

with pkgs;

rustPlatform.buildRustPackage rec {
  name = "evolvim";

  #src = ./evolvim-lib;
  src = ./.;

  verifyCargoDeps = true;
  cargoSha256 = "1ay0j6rz5xykqpzl63kmvj4dzdjkh02pbhfmhwb0vv8c42d56ym3";

  RUST_BACKTRACE = 1;
}
