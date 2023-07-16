# Setting up a new system from scratch

```
git clone https://github.com/andsens/homeshick.git $HOME/.homesick/repos/homeshick
printf '\nsource "$HOME/.homesick/repos/homeshick/homeshick.sh"' >> $HOME/.bashrc
. ~/.bashrc
homeshick clone https://github.com/AmateurECE/dotfiles
```

# Setting up emacs

Initialization will fail the first time Emacs is started, due to missing
packages. The following commands will install the missing packages from my
Emacs package repository:

```
curl -O https://raw.githubusercontent.com/AmateurECE/Emacs-Extensions/master/insert-banner.el
curl -O https://raw.githubusercontent.com/AmateurECE/Emacs-Extensions/master/local-snippet.el
```

In Emacs, install the local packages, and all other required packages:

```
M-x package-install-file insert-banner.el
M-x package-install-file local-snippet.el
M-x package-install-selected-packages
```

# Work patches

To setup a system with the work patches, clone the repository and apply the
patches to a local branch:

```
git clone git@github.com:AmateurECE/dotfiles.git
git checkout -b work
git am work-patches/*.patch
```

Repeat this workflow whenever pulling work from upstream. To update the
patchset, use git-rebase(1). To commit the new patches to version control,
use git-format-patch(1):

```
git format-patch -p -o work-patches/ master..
```
