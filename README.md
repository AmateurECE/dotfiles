# Setting up a new system from scratch

```
git clone https://github.com/andsens/homeshick.git $HOME/.homesick/repos/homeshick
printf '\nsource "$HOME/.homesick/repos/homeshick/homeshick.sh"' >> $HOME/.bashrc
. ~/.bashrc
homeshick clone https://github.com/AmateurECE/dotfiles
```

# Setting up emacs

Initialization will fail the first time Emacs is started, due to missing packages. For
the moment, one can avoid this by installing the `insert-banner` package manually:

```
curl https://raw.githubusercontent.com/AmateurECE/Emacs-Extensions/master/insert-banner.el
```

In Emacs, use the `package-install-file` command to install this package. The file can
then be removed.

TODO: Self-host an ELPA repository and move all ELPA setup to the emacs-bootstrap.el
script, so that the easiest way to install new packages is to add them to this script
and then source it in Emacs.
