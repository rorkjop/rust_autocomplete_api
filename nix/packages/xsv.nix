{
  rustPlatform,
  fetchFromGitHub,
  ...
}:
rustPlatform.buildRustPackage rec {
  pname = "xsv";
  version = "0.13.0";

  src = fetchFromGitHub {
    owner = "BurntSushi";
    repo = pname;
    tag = version;
    hash = "sha256-hYAnLzpOZe//xNWgUxrc9LTAbh2tk+1Ly1nlaga3YZ8=";
  };

  doCheck = false;

  # Needs to use importCargoLock as xsv uses an old lockfile format
  cargoDeps = rustPlatform.importCargoLock {lockFile = "${src}/Cargo.lock";};

  meta = {
    mainProgram = "xsv";
  };
}
