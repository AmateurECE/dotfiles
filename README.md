# Setting up a new system from scratch

```
git clone https://github.com/andsens/homeshick.git $HOME/.homesick/repos/homeshick
printf '\nsource "$HOME/.homesick/repos/homeshick/homeshick.sh"' >> $HOME/.bashrc
. ~/.bashrc
homeshick clone https://github.com/AmateurECE/dotfiles
```

# Desktop Color Palette

The Hyprland desktop theme uses this 10-color palette:

![palette](palette.jpg)

Hex codes are in `palette.txt`.

# Work patches

To setup a system with the work patches, clone the repository and apply the
patches to a local branch:

```
git clone git@github.com:AmateurECE/dotfiles.git
git checkout -b work
git am --no-gpg-sign patches/*.patch
```

Repeat this workflow whenever pulling work from upstream. To update the
patchset, use git-rebase(1) and git-format-patch(1):

```
git rebase origin/master
git format-patch -p -o patches/ master..
```

Since changing the global .gitconfig while performing complicated operations
on a repository (like rebasing, merging, am, etc.) can be difficult or even
dangerous, keep a separate checkout (not managed by homeshick) to perform work
on these patches.
