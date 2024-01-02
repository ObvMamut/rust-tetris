{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
    nativeBuildInputs = [
        pkgs.pkg-config
        pkgs.libudev-zero
        pkgs.alsa-lib

    ];
}

