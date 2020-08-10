{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell rec {
    buildInputs = with pkgs.xorg; [ libX11 libX11.dev libXcursor libXrandr libXi ] ++ [ pkgs.cmake pkgs.libglvnd pkgs.freetype pkgs.expat ];
    nativeBuildInputs = with pkgs; [ pkgconfig alsaLib vulkan-loader ];

    LD_LIBRARY_PATH="${pkgs.stdenv.lib.makeLibraryPath buildInputs}:${pkgs.stdenv.lib.makeLibraryPath nativeBuildInputs}";
}
