let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  shellPackages = with pkgs; [
    toolchain
    rust-analyzer
    clippy
  ];

  libPath = with pkgs; lib.makeLibraryPath shellPackages;
in
pkgs.mkShell {
  packages = shellPackages;
  LD_LIBRARY_PATH = libPath; 
}
