{ lib
, rustPlatform
, pkg-config
, alsa-lib
, libudev-zero
, libX11
, libXcursor
, libXrandr
, libXi
, libGL
, vulkan-loader
, makeWrapper
}:
rustPlatform.buildRustPackage {
  pname = "rust-tetris";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  buildInputs = [
    alsa-lib
    libudev-zero
    libX11
    libXcursor
    libXrandr
    libXi
  ];
  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];

  # Make sure that our program knows where to find graphics libraries at
  # runtime.
  postInstall = ''
    wrapProgram $out/bin/rust-tetris \
      --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath[
        "/run/opengl-driver"
        "/run/opengl-driver-32"
        libGL
        vulkan-loader
      ]}"
  '';
}
