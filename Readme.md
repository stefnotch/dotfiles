# NixOS Config

Following tips from https://nixos-and-flakes.thiscute.world/ and other sources.

## Setup

Clone to your development machine. 
```
jj git clone --colocate git@github.com:stefnotch/dotfiles.git
```

Do the edits. 

And then Nixy Switchy to it
```bash
sudo nixos-rebuild switch --flake ~/dotfiles/homeserver#homeserver
```

## Push directly from my laptop to the homeserver

And push to it
```bash
jj git remote add homeserver ssh://stefnotch@192.168.1.10/~/dotfiles
jj bookmark track main --remote homeserver
jj git push --remote homeserver
```
