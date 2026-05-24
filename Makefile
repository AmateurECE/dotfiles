#!/usr/bin/make -f

all:

datadir=/usr/share
systemd_unitdir=/usr/lib/systemd/system
systemd_userdir=/usr/lib/systemd/user

install:
	install -Dm644 cobblede/greetd.toml \
		"$(DESTDIR)$(datadir)/greetd/greetd.toml"
	install -Dm644 cobblede/greetd.conf \
		"$(DESTDIR)$(systemd_unitdir)/greetd.service.d/greetd.conf"
	install -Dm644 cobblede/settings.ini \
		"$(DESTDIR)/etc/gtk-3.0/settings.ini"
	install -Dm755 cobblede/cobblede.sh -t "$(DESTDIR)/etc/profile.d"
	: # Unit overrides for xdg-desktop-portal services.
	install -Dm644 cobblede/xdg-desktop-portal-gtk.conf \
		-t $(DESTDIR)$(systemd_userdir)/xdg-desktop-portal-gtk.service.d
	install -Dm644 cobblede/xdg-desktop-portal-wlr.conf \
		-t $(DESTDIR)$(systemd_userdir)/xdg-desktop-portal-wlr.service.d
	: # SSH Agent
	install -Dm644 ssh-agent.service -t $(DESTDIR)$(systemd_userdir)
	: # Enable "the notch"
	install -Dm644 cobblede/appledrm.conf -t $(DESTDIR)/lib/modprobe.d
