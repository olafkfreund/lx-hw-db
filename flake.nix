{
  description = "Linux Hardware Database - Privacy-first hardware detection tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        
        lx-hw-db = pkgs.callPackage ./default.nix { };
        
      in {
        packages = {
          default = lx-hw-db;
          lx-hw-db = lx-hw-db;
        };

        devShells.default = pkgs.mkShell {
          name = "lx-hw-db-dev";
          
          packages = with pkgs; [
            # Rust toolchain
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer

            # Build dependencies
            pkg-config
            openssl
            
            # GTK4 development libraries
            gtk4
            libadwaita
            glib
            cairo
            pango
            gdk-pixbuf
            graphene
            gobject-introspection

            # Hardware detection tools
            lshw
            dmidecode
            pciutils
            usbutils
            util-linux
            inxi

            # Packaging tools
            installShellFiles
            makeWrapper
            dpkg
            rpm
          ];

          shellHook = ''
            echo "🚀 Linux Hardware Database Development Environment"
            echo "   GTK4 + libadwaita + Rust development shell"
            echo ""
            echo "📦 Available tools:"
            echo "   • cargo build --bin lx-hw-detect-gtk"
            echo "   • cargo run --bin lx-hw-detect-gtk"  
            echo "   • cargo test"
            echo "   • nix build .#lx-hw-db"
            echo ""
            echo "🔧 Hardware detection tools available:"
            echo "   • lshw, dmidecode, lspci, lsusb, inxi"
            echo ""
            echo "📚 Environment configured for:"
            echo "   • GTK4 $(pkg-config --modversion gtk4)"
            echo "   • libadwaita $(pkg-config --modversion libadwaita-1)"
            echo "   • Rust $(rustc --version | cut -d' ' -f2)"
            echo ""
            echo "✅ Development environment ready!"
          '';

          # Set environment variables for GTK4 development
          GSK_RENDERER = "gl";
          PKG_CONFIG_PATH = "${pkgs.gtk4.dev}/lib/pkgconfig:${pkgs.libadwaita.dev}/lib/pkgconfig";
        };

        apps = {
          default = flake-utils.lib.mkApp {
            drv = lx-hw-db;
            exePath = "/bin/lx-hw-detect";
          };
          
          gui = flake-utils.lib.mkApp {
            drv = lx-hw-db;
            exePath = "/bin/lx-hw-detect-gtk";
          };
        };

        # For nixOS modules
        nixosModules.default = import ./packaging/nixos/module.nix;
        
        # For home-manager modules  
        homeManagerModules.default = import ./packaging/nixos/home-manager.nix;
      }
    );
}