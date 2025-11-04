{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rust-analyzer
    rustfmt
    clippy

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
    echo "Rust development environment"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "Protoc version: $(protoc --version)"
  '';
}
