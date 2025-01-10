{
  description = "VDiDi flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let 
      system = "x86_64-linux";
    in {
      devShells."${system}".default = let 
      pkgs = import nixpkgs {
        inherit system;
      };
      in 
      pkgs.mkShell {
      
          shellHook = ''
            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export LD_LIBRARY_PATH="${pkgs.openssl.out}/lib"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig";
          '';
    };
  };
}
