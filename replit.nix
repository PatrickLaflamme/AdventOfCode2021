{ pkgs }: {
	deps = [
    pkgs.openssl-devel
    pkgs.rustup
		pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
    pkgs.rust-analyzer
	];
}