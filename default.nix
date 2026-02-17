{ 
  pkgs, 
  naerskLib,
  ... 
}:
naerskLib.buildPackage {
    src = ./.;
    buildInputs = [ pkgs.glib ];
    nativeBuildInputs = [ pkgs.pkg-config ];
}
