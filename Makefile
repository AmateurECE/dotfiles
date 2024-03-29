#!/usr/bin/make -f

all:

datadir=/usr/share
systemd_unitdir=/usr/lib/systemd/system
systemd_userdir=/usr/lib/systemd/user

install:
	install -Dm644 wayland/greetd.toml \
		"$(DESTDIR)$(datadir)/greetd/greetd.toml"
	install -Dm644 wayland/greetd.conf \
		"$(DESTDIR)$(systemd_unitdir)/greetd.service.d/greetd.conf"
	install -Dm644 wayland/settings.ini \
		"$(DESTDIR)/etc/gtk-3.0/settings.ini"
	install -Dm755 wayland/cobblede.sh -t "$(DESTDIR)/etc/profile.d"
	: # Unit overrides for xdg-desktop-portal services.
	install -Dm644 wayland/xdg-desktop-portal-gtk.conf \
		-t $(DESTDIR)$(systemd_userdir)/xdg-desktop-portal-gtk.service.d
	install -Dm644 wayland/xdg-desktop-portal-wlr.conf \
		-t $(DESTDIR)$(systemd_userdir)/xdg-desktop-portal-wlr.service.d
	: # SSH Agent
	install -Dm644 ssh-agent.service -t $(DESTDIR)$(systemd_userdir)
	: # Enable "the notch"
	install -Dm644 wayland/apple-dcp.conf -t $(DESTDIR)/lib/modprobe.d
