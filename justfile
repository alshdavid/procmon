set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

build:
  cargo build
fmt:
  cargo +nightly fmt
