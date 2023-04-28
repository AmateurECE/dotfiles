#!/bin/bash
###############################################################################
# NAME:             startwm.sh
#
# AUTHOR:           Ethan D. Twardy <ethan.twardy@gmail.com>
#
# DESCRIPTION:      Simple wrapping script for starting window managers based
#                   on wlroots.
#
# CREATED:          07/22/2022
#
# LAST EDITED:      12/24/2022
###

export XDG_SESSION_TYPE=wayland

STARTWM_SHELL=$1 && shift

# Start sway if no other shell is specified.
: ${STARTWM_SHELL:=sway}

setup_env_sway() {
    export XDG_SESSION_DESKTOP=sway
    export XDG_CURRENT_DESKTOP=sway
}

setup_env_hyprland() {
    export XDG_SESSION_DESKTOP=hyprland
    export XDG_CURRENT_DESKTOP=hyprland
}

start_shell() {
    local shell=$1 && shift
    exec $shell $@
}

case "$STARTWM_SHELL" in
    sway)
        setup_env_sway
        start_shell /usr/bin/sway $@
        ;;
    hyprland)
        setup_env_hyprland
        start_shell /usr/bin/Hyprland $@
        ;;
    *)
        >&2 printf '%s\n' "Unknown window manager $STARTWM_SHELL"
        RC=1
        ;;
esac

###############################################################################
