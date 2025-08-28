{ lib
, rustPlatform
, pkg-config
, openssl
, gtk4
, libadwaita
, glib
, cairo
, pango
, gdk-pixbuf
, graphene
, gobject-introspection
, lshw
, dmidecode
, pciutils
, usbutils
, util-linux
, installShellFiles
, makeWrapper
, wrapGAppsHook4
# Optional dependencies
, inxi ? null
, python3 ? null
}:

rustPlatform.buildRustPackage rec {
  pname = "lx-hw-db";
  version = "0.1.0";

  src = ./.;

  cargoHash = "";

  nativeBuildInputs = [
    pkg-config
    installShellFiles
    makeWrapper
    wrapGAppsHook4
    gobject-introspection
  ];

  buildInputs = [
    openssl
    gtk4
    libadwaita
    glib
    cairo
    pango
    gdk-pixbuf
    graphene
  ];

  # Runtime dependencies that should be available in PATH
  runtimeDeps = [
    lshw
    dmidecode
    pciutils
    usbutils
    util-linux
  ] ++ lib.optionals (inxi != null) [ inxi ]
    ++ lib.optionals (python3 != null) [ python3 ];

  postInstall = ''
    # Install man pages
    installManPage docs/man/lx-hw-detect.1
    installManPage docs/man/lx-hw-indexer.1
    
    # Install shell completions
    installShellCompletion --bash completions/bash/lx-hw-detect
    installShellCompletion --zsh completions/zsh/_lx-hw-detect
    installShellCompletion --fish completions/fish/lx-hw-detect.fish
    
    # Install configuration files
    mkdir -p $out/etc/lx-hw-db
    cp config/default.toml $out/etc/lx-hw-db/config.toml
    
    # Install web interface
    mkdir -p $out/share/lx-hw-db
    cp -r web $out/share/lx-hw-db/
    
    # Install desktop file for GUI
    mkdir -p $out/share/applications
    cat > $out/share/applications/lx-hw-detect.desktop <<EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Linux Hardware Database
Comment=Privacy-preserving hardware detection and compatibility reporting
Exec=$out/bin/lx-hw-detect-gtk
Icon=computer-symbolic
Terminal=false
StartupNotify=true
Categories=System;HardwareSettings;
EOF
    
    # Wrap binaries to ensure runtime dependencies are available
    wrapProgram $out/bin/lx-hw-detect \
      --prefix PATH : ${lib.makeBinPath runtimeDeps}
    wrapProgram $out/bin/lx-hw-indexer \
      --prefix PATH : ${lib.makeBinPath runtimeDeps}
  '';

  # Skip tests that require hardware access
  checkFlags = [
    "--skip=test_hardware_detection"
    "--skip=test_privilege_requirements"
  ];

  meta = with lib; {
    description = "Privacy-first Linux hardware compatibility database tool with GTK4 GUI";
    longDescription = ''
      lx-hw-db is a comprehensive Linux hardware detection and configuration tool
      that helps users ensure optimal hardware compatibility. It provides:

      • Privacy-preserving hardware detection using multiple Linux utilities
      • Modern GTK4 graphical interface with Adwaita design
      • Comprehensive hardware overview with tree-like display
      • Automated configuration recommendations and driver mapping
      • Privacy-first architecture with HMAC-SHA256 anonymization
      • Community-driven hardware compatibility database

      The tool implements comprehensive privacy protection through cryptographic
      anonymization while maintaining statistical utility for compatibility analysis.
    '';
    homepage = "https://github.com/lx-hw-db/lx-hw-db";
    license = licenses.agpl3Plus;
    maintainers = with maintainers; [ ]; # Add maintainer names when available
    platforms = platforms.linux;
    mainProgram = "lx-hw-detect";
  };
}