# NixOS Config

## Setup

Clone to your development machine. Do the edits.

Then create a remote
```bash
ssh stefnotch@192.168.1.10 'git init -b main ~/dotfiles; cd ~/dotfiles; git config --local receive.denyCurrentBranch updateInstead'
```

And push to it
```bash
jj git remote add homeserver ssh://stefnotch@192.168.1.10/~/dotfiles
jj bookmark track main --remote homeserver
jj git push -b main --remote homeserver
```

