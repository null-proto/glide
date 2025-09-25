{
  description = "A Git repo hoster ";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
	  system = "x86_64-linux";
		pkgs = nixpkgs.legacyPackages.${system};
	in {

		devShells.${system} = {
			latest-stable-fish = pkgs.mkShell {
				buildInputs = with pkgs; [
					rustup
				];

				shellHook = ''
					rustup default stable
					export SHELL=${builtins.getEnv "SHELL"}
					export EDITOR=${pkgs.neovim}/bin/nvim
					exec $SHELL
				'';
			};

			rust-lts-fish = pkgs.mkShell {
				buildInputs = with pkgs; [
					rustc
					rustfmt
					rust-analyzer
					cargo
				];

				shellHook = ''
					export SHELL=${pkgs.fish}/bin/fish
					export EDITOR=${pkgs.neovim}/bin/nvim
					exec $SHELL
				'';
			};
		};
  };
}
