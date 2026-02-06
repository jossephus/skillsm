{
  description = "Skillsm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    {
      overlay = final: prev: {
        inherit (self.packages.${final.system}) android-sdk;
      };
    }
    // flake-utils.lib.eachSystem ["x86_64-linux" "aarch64-darwin"] (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [
            self.overlay
            (import rust-overlay)
          ];
        };
      in {
        devShell = pkgs.mkShell {
          buildInput = [
            pkgs.rust-bin.stable.latest.default
            pkgs.rust-analyzer
          ];
        };
      }
    );
}
