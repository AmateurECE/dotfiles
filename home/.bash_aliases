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
# LAST EDITED:	    04/30/2023
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

# DEB* - Used by various debian package maintainer scripts
# GIT_* - Used by git commands
# EMAIL - Also used by other git commands
export EMAIL=ethan.twardy@gmail.com
export GIT_AUTHOR_EMAIL=$EMAIL
export GIT_COMMITTER_EMAIL=$EMAIL
export DEBEMAIL=$EMAIL

export GIT_AUTHOR_NAME="Ethan D. Twardy"
export GIT_COMMITTER_NAME=$GIT_AUTHOR_NAME
export DEBFULLNAME=$GIT_AUTHOR_NAME

export ECHANGELOG_USER="$GIT_AUTHOR_NAME <$EMAIL>"

# Useful on systems where I run Python.
if [[ -d $HOME/.local/bin ]]; then
    export PATH="$HOME/.local/bin:$PATH"
fi

# Useful on systems where I run Rust.
if [[ -d $HOME/.cargo/bin ]]; then
    export PATH="$HOME/.cargo/bin:$PATH"
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
if [[ $(systemctl --user is-active ssh-agent.service) = "active" ]]; then
    export SSH_AUTH_SOCK="$XDG_RUNTIME_DIR/ssh-agent.socket"
fi

###############################################################################
