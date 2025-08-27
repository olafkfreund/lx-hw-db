Name:           lx-hw-db
Version:        0.1.0
Release:        1%{?dist}
Summary:        Privacy-first Linux hardware compatibility database tool

License:        AGPL-3.0-or-later
URL:            https://github.com/lx-hw-db/lx-hw-db
Source0:        https://github.com/lx-hw-db/lx-hw-db/archive/v%{version}.tar.gz#/%{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  pkgconfig
BuildRequires:  openssl-devel
BuildRequires:  systemd-rpm-macros

Requires:       lshw
Requires:       dmidecode
Requires:       pciutils
Requires:       usbutils
Requires:       util-linux

Recommends:     inxi
Recommends:     python3
Suggests:       curl
Suggests:       git

%description
lx-hw-db is a community-driven hardware detection and configuration tool
that helps Linux users ensure optimal hardware compatibility. It provides
privacy-preserving hardware detection using multiple Linux utilities,
automated configuration recommendations, driver mapping, kernel parameter
optimization, and DKMS module management.

The tool implements comprehensive privacy protection through cryptographic
anonymization while maintaining statistical utility for compatibility analysis.

%prep
%autosetup -n %{name}-%{version}

%build
export RUSTFLAGS="%{build_rustflags}"
cargo build --release --bins

%install
# Install binaries
install -Dm755 target/release/lx-hw-detect %{buildroot}%{_bindir}/lx-hw-detect
install -Dm755 target/release/lx-hw-indexer %{buildroot}%{_bindir}/lx-hw-indexer

# Install configuration
install -Dm644 config/default.toml %{buildroot}%{_sysconfdir}/lx-hw-db/config.toml

# Install documentation
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE

# Install man pages
install -Dm644 docs/man/lx-hw-detect.1 %{buildroot}%{_mandir}/man1/lx-hw-detect.1
install -Dm644 docs/man/lx-hw-indexer.1 %{buildroot}%{_mandir}/man1/lx-hw-indexer.1

# Install web interface
mkdir -p %{buildroot}%{_datadir}/%{name}
cp -r web %{buildroot}%{_datadir}/%{name}/

# Install systemd service
install -Dm644 packaging/systemd/lx-hw-db-server.service \
    %{buildroot}%{_unitdir}/lx-hw-db-server.service

# Install shell completions
install -Dm644 completions/bash/lx-hw-detect \
    %{buildroot}%{_datadir}/bash-completion/completions/lx-hw-detect
install -Dm644 completions/zsh/_lx-hw-detect \
    %{buildroot}%{_datadir}/zsh/site-functions/_lx-hw-detect
install -Dm644 completions/fish/lx-hw-detect.fish \
    %{buildroot}%{_datadir}/fish/vendor_completions.d/lx-hw-detect.fish

%check
cargo test --release

%post
%systemd_post lx-hw-db-server.service

%preun
%systemd_preun lx-hw-db-server.service

%postun
%systemd_postun_with_restart lx-hw-db-server.service

%files
%license LICENSE
%doc README.md
%{_bindir}/lx-hw-detect
%{_bindir}/lx-hw-indexer
%config(noreplace) %{_sysconfdir}/lx-hw-db/config.toml
%{_mandir}/man1/lx-hw-detect.1*
%{_mandir}/man1/lx-hw-indexer.1*
%{_datadir}/%{name}/
%{_unitdir}/lx-hw-db-server.service
%{_datadir}/bash-completion/completions/lx-hw-detect
%{_datadir}/zsh/site-functions/_lx-hw-detect
%{_datadir}/fish/vendor_completions.d/lx-hw-detect.fish

%changelog
* Wed Aug 27 2025 Linux Hardware Database Project <maintainers@lx-hw-db.org> - 0.1.0-1
- Initial package for Fedora
- Privacy-first hardware detection with cryptographic anonymization
- Configuration engine with driver recommendations
- Web interface and CLI tools
- DKMS module management support