{
  description = "Knowledge manager, CLI tool to manage knowledge in hierarchical structure";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    {
      # Overlay so the package can be used in NixOS system flake
      overlays.default = final: prev: {
        knowledge-manager = self.packages.${final.stdenv.hostPlatform.system}.knowledge-manager;
        km = self.packages.${final.stdenv.hostPlatform.system}.km;
      };
    }
    // (flake-utils.lib.eachSystem nixpkgs.lib.systems.flakeExposed (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };
      in
      {
        formatter = pkgs.alejandra;

        packages = {
          knowledge-manager = pkgs.rustPlatform.buildRustPackage {
            pname = "knowledge-manager";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          km = pkgs.runCommand "km" { } ''
            mkdir -p $out/bin
            ln -s ${self.packages.${system}.knowledge-manager}/bin/knowledge-manager $out/bin/km
          '';

          default = self.packages.${system}.knowledge-manager;
        };

        devShells.default = pkgs.mkShell {
          name = "knowledge-manager";
          packages = with pkgs; [
            rustToolchain
            cargo-nextest
            cargo-deny
            bacon
            # `km` command available inside devShell
            (pkgs.writeShellScriptBin "km" ''
              exec ${self.packages.${system}.knowledge-manager}/bin/knowledge-manager "$@"
            '')
          ];
          RUSTFLAGS = "-Zthreads=0";
          shellHook = ''
            echo "🦀 knowledge-manager devShell (try: km --help)"
          '';
        };
      }
    ));
}
