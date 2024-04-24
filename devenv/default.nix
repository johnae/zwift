{
  inputs,
  pkgs,
  ansiEscape,
  ...
}: let
  rustVersion = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
in rec {
  name = "Zwift";
  languages.rust.enable = true;
  languages.rust.toolchain.rustc = rustVersion.override {
    extensions = ["rust-src"];
    targets = ["wasm32-unknown-unknown" "wasm32-wasi"];
  };
  languages.nix.enable = true;
  packages = with pkgs; [
    alejandra
    taplo
    libiconv
    wasmtime
  ];
  enterShell = ansiEscape ''
     echo -e "
      {bold}{160}${name}{reset}

      Zwift for Zellij
    "
  '';
}
