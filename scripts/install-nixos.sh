#!/bin/bash
# NixOS specific installation script for Linux Hardware Database

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}Installing Linux Hardware Database on NixOS...${NC}"

# Check if we're in the repository
if [[ -f "flake.nix" ]]; then
    echo -e "${BLUE}Found flake.nix, installing from current directory...${NC}"
    
    echo -e "${BLUE}Installing with nix profile...${NC}"
    nix profile install .#lx-hw-db
    
    echo -e "${GREEN}Installation complete!${NC}"
    echo
    echo "Usage:"
    echo "  lx-hw-detect --help          # Show help"
    echo "  lx-hw-detect detect          # Detect hardware"
    echo "  lx-hw-indexer --help         # Show indexer help"
    
else
    echo -e "${BLUE}Installing from GitHub repository...${NC}"
    
    # Install from GitHub
    nix profile install github:lx-hw-db/lx-hw-db
    
    echo -e "${GREEN}Installation complete!${NC}"
fi

echo
echo -e "${YELLOW}NixOS Configuration Integration:${NC}"
echo
echo "To integrate with your NixOS configuration, add to your flake.nix:"
echo
cat << 'EOF'
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    lx-hw-db.url = "github:lx-hw-db/lx-hw-db";
  };
  
  outputs = { self, nixpkgs, lx-hw-db }: {
    nixosConfigurations.yourhost = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        lx-hw-db.nixosModules.default
        {
          services.lx-hw-db.enable = true;
        }
      ];
    };
  };
}
EOF

echo
echo "For home-manager integration:"
echo
cat << 'EOF'
{
  inputs.lx-hw-db.url = "github:lx-hw-db/lx-hw-db";
  
  outputs = { self, nixpkgs, home-manager, lx-hw-db }: {
    homeConfigurations.username = home-manager.lib.homeManagerConfiguration {
      modules = [
        lx-hw-db.homeManagerModules.default
        {
          programs.lx-hw-db.enable = true;
        }
      ];
    };
  };
}
EOF

echo
echo -e "${BLUE}Development shell available with: nix develop${NC}"