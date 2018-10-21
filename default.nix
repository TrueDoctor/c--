with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "interfuck";
  src = if lib.inNixShell then null else ./.;

  makeFlags = [ "PREFIX=$(out)" ];

  preferLocalBuild = true;
}
