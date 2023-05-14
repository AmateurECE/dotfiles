#!/usr/bin/make -f

all:

datadir=/usr/share
systemd_unitdir=/usr/lib/systemd/system
systemd_userdir=/usr/lib/systemd/user

install:
	install -Dm644 wayland/greetd.toml \
		"$(DESTDIR)$(datadir)/greetd/greetd.toml"
	install -Dm644 wayland/environments.conf \
		"$(DESTDIR)/etc/greetd/environments"
	install -Dm644 wayland/sway.conf "$(DESTDIR)/etc/greetd/sway-config"
	install -Dm644 wayland/gtkgreet.css "$(DESTDIR)/etc/greetd/gtkgreet.css"
	install -Dm644 wayland/greetd.conf \
		"$(DESTDIR)$(systemd_unitdir)/greetd.service.d/greetd.conf"
	install -Dm644 wayland/settings.ini \
		"$(DESTDIR)/etc/gtk-3.0/settings.ini"
	install -Dm755 wayland/startwm.sh "$(DESTDIR)/usr/bin/startwm"
	install -Dm644 wayland/xdg-desktop-portal-gtk.conf \
		-t $(DESTDIR)$(systemd_userdir)/xdg-desktop-portal-gtk.service.d
	install -Dm644 wayland/xdg-desktop-portal-wlr.conf \
		-t $(DESTDIR)$(systemd_userdir)/xdg-desktop-portal-wlr.service.d
