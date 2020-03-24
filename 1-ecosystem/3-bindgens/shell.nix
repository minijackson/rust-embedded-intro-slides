let
  pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
  buildInputs = [
    #pkgs.llvmPackages.libclang
  ];

  LLVM_CONFIG_PATH = "${pkgs.llvmPackages.llvm}/bin/llvm-config";
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
}
