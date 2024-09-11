{inputs, ...}: {
  perSystem = {
    system,
    pkgs,
    ...
  }: let
    rustWasi = pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-src" "rust-std"];
      targets = ["wasm32-wasi"];
    };
    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustWasi;
  in {
    packages.default = craneLib.buildPackage {
      src = craneLib.cleanCargoSource (craneLib.path ../.);
      cargoExtraArgs = "--target wasm32-wasi";
      doCheck = false;
    };
  };
}
