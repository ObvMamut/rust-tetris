{ pkgs ? import <nixpkgs> { }
, rust-tetris ? pkgs.callPackage ./build.nix { }
}:
with pkgs;
mkShell
{
  # Pull in everything needed to build our crate.
  inputsFrom = [ rust-tetris ];

  # Other development niceties.
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  packages = with pkgs; [
    rust-analyzer
    clippy
  ];

  # Make sure the graphics libraries are findable in our dev shell.
  shellHook = ''
    export LD_LIBRARY_PATH="${lib.makeLibraryPath [
      "/run/opengl-driver"
      "/run/opengl-driver-32"
      libGL
      vulkan-loader
    ]}:$LD_LIBRARY_PATH"
  '';
}

