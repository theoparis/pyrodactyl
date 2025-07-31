{
  self,
  rustPlatform,
}:
{
  pyrodactyl-api = rustPlatform.buildRustPackage {
    name = "pyrodactyl-api";
    version = "4.0.0-dev";
    src = self;

    buildAndTestSubdir = "rewrite/pyrodactyl-api";
    cargoLock.lockFile = ./Cargo.lock;

    buildInputs = [
    ];
  };
}
