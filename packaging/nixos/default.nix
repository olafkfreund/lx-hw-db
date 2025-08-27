{ lib
, rustPlatform
, fetchFromGitHub
, pkg-config
, openssl
, lshw
, dmidecode
, pciutils
, usbutils
, util-linux
, installShellFiles
, makeWrapper
# Optional dependencies
, inxi ? null
, python3 ? null
}:

rustPlatform.buildRustPackage rec {
  pname = "lx-hw-db";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "lx-hw-db";
    repo = "lx-hw-db";
    rev = "v${version}";
    hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # Will be updated with actual hash
  };

  cargoHash = "sha256-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB="; # Will be updated with actual hash

  nativeBuildInputs = [
    pkg-config
    installShellFiles
    makeWrapper
  ];

  buildInputs = [
    openssl
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
    description = "Privacy-first Linux hardware compatibility database tool";
    longDescription = ''
      lx-hw-db is a community-driven hardware detection and configuration tool
      that helps Linux users ensure optimal hardware compatibility. It provides
      privacy-preserving hardware detection using multiple Linux utilities,
      automated configuration recommendations, driver mapping, kernel parameter
      optimization, and DKMS module management.

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