# Setting up a new NixOS machine

Here's what the baseline config needs.
```
 # Nix Flakes Support
nix.settings.experimental-features = [ "nix-command" "flakes" ];

# List packages installed in system profile. To search, run:
# $ nix search wget
environment.systemPackages = with pkgs; [
  git # Flakes need this
  jujutsu
];
```

Then

```bash
sudo nixos-rebuild switch
```

And for GitHub, we need a ssh key.
```
ssh-keygen -t ed25519 -C "stefnotch@users.noreply.github.com"
cat ~/.ssh/id_ed25519.pub
```

After GitHub, we need the jj configs
`jj config edit --user

```
#:schema https://docs.jj-vcs.dev/latest/config-schema.json

[user]
name = "Stefnotch"
email = "stefnotch@users.noreply.github.com"

[ui]
default-command = "log"

[git]
colocate = false

[working-copy]
eol-conversion = "none"

[revsets]
bookmark-advance-to = "closest_pushable(@)"

[revset-aliases]
# Closest revision that is mutable, described and either non-empty or a merge
'closest_pushable(to)' = '''
  heads(::to & mutable() & ~description(exact:"") & (~empty() | merges()))
'''
```

At the end, delete your `/etc/nixos/configuration.nix`.
