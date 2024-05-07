let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = import nixpkgs { config = {}; overlays = [ rust-overlay ]; };
  system = builtins.currentSystem;
  extensions =
    (import (builtins.fetchGit {
      url = "https://github.com/nix-community/nix-vscode-extensions";
      ref = "master";
      rev = "926cfa719fef55c3dab86d19b41fd8ee1fc8ea21";
    })).extensions.${system};
  extensionsList = with extensions.vscode-marketplace; [
      rust-lang.rust-analyzer
      tamasfe.even-better-toml
      usernamehw.errorlens
      serayuzgur.crates
      vadimcn.vscode-lldb
  ];
  buildInputs = with pkgs; [
    udev
    alsa-lib
    libglvnd
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    libxkbcommon
    wayland
  ];
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      pkg-config
    ];
    inherit buildInputs;
    packages = with pkgs; [
      git
      (rust-bin.stable.latest.default.override { extensions = ["rust-src"]; })
      (vscode-with-extensions.override {
        vscode = vscodium;
        vscodeExtensions = extensionsList;
      })
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    NIXOS_OZONE_WL=1;
  }
