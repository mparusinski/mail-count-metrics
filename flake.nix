{
  description = "Mail counter simple";
  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux"; # your version
     pkgs = nixpkgs.legacyPackages.${system};    
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        packages = with pkgs; [ rustc cargo pkg-config openssl ]; # whatever you need
      };
    };
}
