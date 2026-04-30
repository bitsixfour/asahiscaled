{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.git-credential-manager
    pkgs.openssl
    pkgs.pkg-config
    pkgs.dbus
  ];
}
