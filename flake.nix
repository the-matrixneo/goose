{
  description = "goose - An AI agent CLI";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        
        # Read package metadata from Cargo.toml
        cargoToml = builtins.fromTOML (builtins.readFile ./crates/goose-cli/Cargo.toml);
        workspaceToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        
        commonInputs = [
          rust
          pkgs.rust-analyzer
          pkgs.pkg-config
          pkgs.openssl
        ];
        
        darwinInputs = with pkgs; [
          libiconv
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
          darwin.apple_sdk.frameworks.CoreServices
        ];
        
        buildInputs = commonInputs
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin darwinInputs;
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = workspaceToml.workspace.package.version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              # Patch required for Windows cross-compilation
              # See: https://github.com/nmathewson/crunchy/tree/cross-compilation-fix
              "crunchy-0.2.3" = "sha256-CBW3/JuMoNa6MWia6BQo07LQrH5JQbb20vuCqhyFL0Y=";
            };
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
            xorg.libxcb  # Required for xcap screenshot functionality
            dbus         # Required for system integration features
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin darwinInputs;

          # Build only the CLI package
          cargoBuildFlags = [ "--package" "goose-cli" ];
          
          # Enable tests with proper environment
          # Tests need writable HOME and XDG directories for config/cache access
          doCheck = true;
          checkPhase = ''
            export HOME=$(mktemp -d)
            export XDG_CONFIG_HOME=$HOME/.config
            export XDG_DATA_HOME=$HOME/.local/share
            export XDG_STATE_HOME=$HOME/.local/state
            export XDG_CACHE_HOME=$HOME/.cache
            mkdir -p $XDG_CONFIG_HOME $XDG_DATA_HOME $XDG_STATE_HOME $XDG_CACHE_HOME
            
            # Run tests for goose-cli package only
            cargo test --package goose-cli --release
          '';

          meta = with pkgs.lib; {
            description = workspaceToml.workspace.package.description;
            homepage = workspaceToml.workspace.package.repository;
            license = licenses.asl20;  # Maps from "Apache-2.0" in Cargo.toml
          };
        };

        devShell = pkgs.mkShell {
          packages = buildInputs ++ (with pkgs; [
            cargo-watch
            cargo-edit
            clippy
            gemini-cli # potentially useful during dev/testing
            go_1_25 # 'just' run-ui (temporal-service)
            just # used in dev/test
            nodejs_24 # 'just' run-ui
            ripgrep
            rustfmt
            xorg.libxcb
            dbus
            yarn # 'just' install-deps
          ]);
          
          shellHook = ''
            echo "goose development environment"
            echo "Rust version: $(rustc --version)"
            echo ""
            echo "Commands:"
            echo "  nix build           - Build goose CLI"
            echo "  nix run             - Run goose CLI"
            echo "  cargo build -p goose-cli - Build with cargo"
            echo "  cargo run -p goose-cli   - Run with cargo"
          '';
        };
      }
    );
}