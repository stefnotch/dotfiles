{
  makeRustPlatform,
  binaryen,
  breakpointHook,
  makeWrapper,

  rustToolchain,
  wasmBindgen,
  dioxus-cli,
}:
let
  name = "homepage";
in
(makeRustPlatform {
  cargo = rustToolchain;
  rustc = rustToolchain;
}).buildRustPackage
  {
    pname = name;
    version = "0.1.0";

    src = ../.;

    nativeBuildInputs = [
      wasmBindgen
      binaryen
      dioxus-cli
      breakpointHook
      makeWrapper
    ];

    buildPhase = ''
      dx bundle --debug-symbols=false --release --web --package ${name}
    '';

    checkPhase = "";

    installPhase = "
      mkdir -p $out/bin
      mkdir -p $out/share

      cp -r target/dx/${name}/release/web $out/share/web

      makeWrapper $out/share/web/server $out/bin/${name} --chdir $out/share/web
    ";

    cargoLock.lockFile = ../Cargo.lock;
  }
