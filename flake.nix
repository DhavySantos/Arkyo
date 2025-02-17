{
  description = "Arkyo Http Server";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@ { self, nixpkgs, flake-parts, ... } :
  flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" ];
    perSystem = { pkgs, system, ... } : {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          clippy bacon rust-analyzer
          cargo rustc rustfmt cmake
          cargo-watch
        ];
      };
    };
  };
}

