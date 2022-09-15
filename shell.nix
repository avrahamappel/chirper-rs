{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.diesel-cli
    pkgs.libiconv
  ];
}
