# Setting up a new system from scratch

```
git clone https://github.com/andsens/homeshick.git $HOME/.homesick/repos/homeshick
printf '\nsource "$HOME/.homesick/repos/homeshick/homeshick.sh"' >> $HOME/.bashrc
. ~/.bashrc
homeshick clone https://github.com/AmateurECE/dotfiles
```

# Setting up emacs

Initialization will fail the first time Emacs is started, due to missing
packages. For The following command can be used to install the `insert-banner`
package. Replacing `insert-banner` with `local-snippet` in the following
command can be used to install that package.

```
curl https://raw.githubusercontent.com/AmateurECE/Emacs-Extensions/master/insert-banner.el
```

In Emacs, use the `package-install-file` command to install this package. The
file can then be removed.

To install all other packages, run the `package-install-selected-packages`
command.
