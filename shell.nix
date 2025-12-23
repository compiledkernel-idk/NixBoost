{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    pkg-config
    openssl
  ];

  shellHook = ''
    echo ":: Welcome to the nixboost development shell"
    echo ":: Dependencies: cargo, rustc, pkg-config, openssl"
  '';
}
