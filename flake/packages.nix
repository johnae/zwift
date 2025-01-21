{inputs, ...}: {
  perSystem = {
    system,
    pkgs,
    ...
  }: let
    rustWasi = pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-src" "rust-std"];
      targets = ["wasm32-wasip1"];
    };
    craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustWasi;
  in {
    packages.default = craneLib.buildPackage {
      src = craneLib.cleanCargoSource (craneLib.path ../.);
      cargoExtraArgs = "--target wasm32-wasip1";
      doCheck = false;
    };
  };
}
