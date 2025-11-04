{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain - uses rust-toolchain.toml which specifies nightly
    rustup

    # Build dependencies
    protobuf
    pkg-config
    openssl

    # For cargo-edit (used in release script)
    cargo-edit
  ];

  # Environment variables
  RUST_BACKTRACE = "1";
  PROTOC = "${pkgs.protobuf}/bin/protoc";
  PROTOC_INCLUDE = "${pkgs.protobuf}/include";

  shellHook = ''
    # Install stable toolchain if not present
    rustup toolchain install stable --component rustfmt clippy rust-analyzer
    rustup default stable

    echo "Rust development environment"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "Protoc version: $(protoc --version)"
  '';
}
