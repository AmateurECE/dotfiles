#!/bin/bash
###############################################################################
# NAME:		    .bash_aliases
#
# AUTHOR:	    Ethan D. Twardy
#
# DESCRIPTION:	    Useful aliases (portable to all Linux systems, or at least
#                   that's the intent).
#
# CREATED:	    10/23/2017
#
# LAST EDITED:	    05/02/2023
###

# Shell prompt
GREEN='\033[32;1m'
BLUE='\033[34;1m'
RESET='\033[0m'
PS1="[\[${GREEN}\]\u@\h \[${BLUE}\]\W\[${RESET}\]]\$ "

# Standard aliases
alias ls='ls -A --color=auto'
alias ll='ls -lA'
alias cp='cp -i'
alias mv='mv -i'
alias rm='rm -i'
alias svn='svn --no-auth-cache'
alias tmux="tmux -f $HOME/.tmux.conf"

export EMAIL=ethan.twardy@gmail.com
export FULLNAME="Ethan D. Twardy"

# Used by various debian package maintainer tools
export DEBEMAIL=$EMAIL
export DEBFULLNAME=$FULLNAME

# Used by ebuild tools (Gentoo)
export ECHANGELOG_USER="$FULLNAME <$EMAIL>"

# Configure a TTY for OpenPGP to prompt for a password on
export GPG_TTY=$(tty)

# Useful on systems where I run Python.
if [[ -d $HOME/.local/bin ]]; then
	export PATH="$HOME/.local/bin:$PATH"
fi

# Useful on systems where I run Rust.
if [[ -d $HOME/.cargo/bin ]]; then
	export PATH="$PATH:$HOME/.cargo/bin"
fi

# Useful on systems where I run Ruby 3.0.0+
if [[ -d $HOME/.local/share/gem/ruby/3.0.0/bin ]]; then
	export PATH="$HOME/.local/share/gem/ruby/3.0.0/bin:$PATH"
fi

# Useful on systems where I run RVM
if [[ -f $HOME/.rvm/scripts/rvm ]]; then
	source $HOME/.rvm/scripts/rvm
fi

# Setup JAVA_HOME on Arch Linux
if grep -q 'Arch Linux' /etc/os-release; then
	export JAVA_HOME=/usr/lib/jvm/default
fi

# Configure ssh-agent for Arch Linux
if [[ -e "$XDG_RUNTIME_DIR/ssh-agent.socket" ]]; then
	export SSH_AUTH_SOCK="$XDG_RUNTIME_DIR/ssh-agent.socket"
elif [[ -e "$XDG_RUNTIME_DIR/openssh_agent" ]]; then
	export SSH_AUTH_SOCK="$XDG_RUNTIME_DIR/openssh_agent"
fi

# Setup direnv, if direnv is installed
if hash direnv 2>/dev/null; then
	eval "$(direnv hook bash)"
fi

# Include programs installed with the nix profile
if [[ -d "$HOME/.nix-profile/bin" ]]; then
	export PATH="$HOME/.nix-profile/bin:$PATH"
fi
