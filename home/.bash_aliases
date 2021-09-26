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
# LAST EDITED:	    09/26/2021
###

# Shell prompt
PS1='[\u@\h \W]\$ '

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

# Useful on systems where I run Python.
if [[ -d $HOME/.local/bin ]]; then
    export PATH="$HOME/.local/bin:$PATH"
fi

###############################################################################
