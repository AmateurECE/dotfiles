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
