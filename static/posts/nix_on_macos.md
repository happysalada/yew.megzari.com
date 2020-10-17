# Nix on MacOS

[Nix](https://github.com/NixOS/nix) is a purely functional package manager, an alternative to HomeBrew.

Pros:

- Possibility to create isolated development environments. (no need for docker for development anymore)
- Replicable builds
- No more breaking updates
- Space efficient! Each installation keeps a dependency tree and when a dependency is no longer needed it is removed.
I'm sure it's what homebrew was supposed to do, but after 5 years there were so many packages I had no idea where they come from on my system.

Cons:

- Learning curve is a little steep. Nix uses it's own language to define packages. There is also a whole eco-system around Nix (feels a little bit like drinking from the hose at first)
- Documentation and examples are not so easy to find
- Update for a new version of package might take a little longer than other package managers to be available

The Nix package manager by itself is not really interesting, it starts to make full sense combining it with [nix-darwin](https://github.com/LnL7/nix-darwin) or [home-manager](https://github.com/nix-community/home-manager) or both!
Those projects enable you to build your system from a configuration file.
Nix-darwin aims at configuring your MacOS (auto-hide the dock, enable 3 finger drag for trackpad...) and define some programs that should available for every users.
Home manager goes one step further and enables you to define the configurations for those programs you install (no more messing with dotfiles and symlinks).

## Should you do it?
I would say you should be ready to invest a good weekend.
Removing HomeBrew and switching completely to Nix freed 40 Gb of space for me.
For development, not having to mess again with docker was a huge advantage too.
If you don't want to use Nix for everything, you can still install binaries the old way.
I still haven't switched my firefox and vscode installation (those are still the old binaries downloaded from the internet).

## How

### Nix

Instruction for installation can be found [here](https://nixos.org/manual/nix/stable/#sect-macos-installation)
The installation can be multi user or single user.
TLDR: only use single user when you are a nix expert.
First pitfall! The default command for macos is for a single user install.
You have to add `--daemon` at the end of it.
Failing to do that, nix will sometimes complain that you don't have permission.
You will either need to sudo or `chown` the `/nix` directory. 

### Nix-darwin

This step is not mandatory. You can use nix without nix-darwin.
If you want to install nix-darwin though, you have to do it before home-manager, otherwise it will cause conflicts.

Pros:
- control all your macos settings with a nix config file (where the dock should be, how fast keyrepeat should be...)
- install applications available to every user of your system
- define services to run on startup (launchd)
- define users on your system and their config
- manage fonts on your system with a config file

Cons:
- Some things won't work smoothly (at least for me it didn't). Installing an app using nix-darwin won't give you the shortcut icon in the Application folder.
- If you are using fish shell, you'll need a shell init script to fix your `$PATH` and `$NIX_PATH`
- there can be some conflicts with home-manager (in the nix version used for example)
- there can be some overlap with home-manager (you can also define packages that you want installed in home-manager)

here is my [nix-darwin config](https://github.com/happysalada/dotfiles/blob/master/nix/darwin.nix) for reference if you want to have a look at it.

here is the [nix-darwin manual](https://daiderd.com/nix-darwin/manual/index.html) with all the configuration options available

#### Installing

check the [intructions](https://github.com/LnL7/nix-darwin)

One caveat is that once you've installed it, you are using the default configuration.
the default configuration is at `~/.nixpkgs/darwin-configuration.nix`. You can copy it or start from somebody else's.
once you've got your new config, you will need to `darwin-rebuild switch -I "darwin-config=PATH_TO_YOUR_CONFIG`
then in your config, you'll need to set the `environment.darwinConfig` variable.

To verify your installation, check `echo $PATH`, it should include `/run/current-system/sw/bin`.
That's where all the binaries installed by nix-darwin will be.
If you find that the path isn't included, when you add home-manager later, you can add a shell script to fix it.
my fix for fish shell is [here](https://github.com/happysalada/dotfiles/blob/master/nix/programs/fish.nix#L25)

#### Usage

after you've modified your config file, run `darwin-rebuild switch` to switch to the new version of your config.
If something breaks, it will tell you and won't make the switch.
No more broken system!
(the worse that can happen with this is that you don't have the latest version of what you want.)

if you want to update packages run `nix-channel --update` then `darwin-rebuild switch`.

you can check out all the packages available at `https://search.nixos.org/packages`.
Some of them won't be available for MacOS, it will tell you if you have them in your config and you try to `darwin-rebuild switch`.

### Home-manager

The promise that home-manager gives you is that you can configure the software you install with a nix file.
If you still want to keep your old config file, you can though (I kept my tmux.conf since I couldn't be bothered to change it and test it).

for reference here is [my config](https://github.com/happysalada/dotfiles/blob/master/nix/home.nix).

The advantage you have is that if you need a shell integration, you just have to add `enableFishIntegration = true;` and you're off to the races (bash or zsh integration are available as well).
No more messing with your .bashrc yourself and then trying to figure ou why the software is not properly integrated (fuzzy finders are an example, fzf or skim).

#### Installing

head over to the [instructions](https://github.com/nix-community/home-manager#installation)

right off the bat you need to choose a stable channel or if you want to be on master.
I found personally that master is stable, and even in the event of a bug, it will be fixed pretty quickly. The project has been fairly active. New packages are added all the time, master is where you get them quickly.

One caveat, is that if you have installed nix-darwin, you can't ask nix-darwin to install home-manager. If you do that then you'll have two versions installed. So leaving home-manager out of nix-darwin config and relying on the master channel of home-manager is the way to go in my opinion.

modify the default config file at `$HOME/.config/nixpkgs/home.nix` to your liking, or be sure to set the `programs.home-manager.path` variable.

#### Choosing a version of nixpkgs.

all the packages you install will depend on which version of nixpkgs you choose.
I don't recommend stable as it can take quite a while for something to get updated.
For example postgres_13 has been added to unstable on 2020-09-24 while it's still not available on latest stable channel.
(you might not want to use the latest and greatest everytime, but that is another discussion altogether.)

#### Usage

after making a modification to your config file run `home-manager switch` to update. If something breaks, it will tell you and won't make the switch to the new configuration.

to update run `nix-channel --update` and `home-manager switch` to get the latest versions of the packages available.

check out all the packages available for home-manager by running `man home-configuration.nix`

### Nix-shell

The promise here is that you can get a list of dependencies available for a particular project.
You don't need docker anymore!
The caveat here is that writing a shell.nix is not trivial, but you can use files from more experienced people.

#### Installation

nix-shell is available with nix, however there are a couple of tools you need to make the integration smoother.
[direnv](https://direnv.net/) will allow you to load environment variables when you cd into a directory.
to enable it, head over to your home-manager config file and add (replace with your favorite shell)
```
programs.direnv = {
    enable = true;
    enableFishIntegration = true;
    enableNixDirenvIntegration = true;
};
```

#### Usage

now inside a dev project you will need two files `.envrc` and `shell.nix`
the `envrc` contains
```
use_nix
```

and the shell.nix will contain the packages you want available.
Here is an example of the [shell.nix](https://github.com/happysalada/dotfiles/blob/master/nix/shells/elixir.nix) I use for an `Elixir` and `Phoenix` project.
[here](https://github.com/happysalada/dotfiles/blob/master/nix/shells/clojure.nix) is one for clojure.
I didn't make those files myself and just took them from wiser people than me.

The first time you `cd` into your dev project you will need to run `direnv allow`.
And then, the dependencies you need are available in that directory.

You will need last to integrate it with your editor.
For that you'll have to find the related `direnv` plugin and take it from there.
