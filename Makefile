#
# IBus Buffalo - A Vietnamese Input Method Engine
# Copyright (C) 2026 Hai Pham Ngoc
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#

PREFIX ?= /usr
DESTDIR ?=

engine_name    = buffalo
pkg_name       = ibus-$(engine_name)
version        = 0.2.0

engine_dir     = $(PREFIX)/share/$(pkg_name)
ibus_dir       = $(PREFIX)/share/ibus
lib_dir        = $(PREFIX)/lib/$(pkg_name)
app_dir        = $(PREFIX)/share/applications

CARGO          ?= cargo
CARGO_FLAGS    ?= --release

# ─────────────────── Targets ────────────────────

.PHONY: all build install uninstall clean

all: build

build:
	$(CARGO) build $(CARGO_FLAGS) -p ibus-buffalo
	$(CARGO) build $(CARGO_FLAGS) -p buffalo-ui

clean:
	$(CARGO) clean

install:
	# Create directories
	install -d $(DESTDIR)$(lib_dir)
	install -d $(DESTDIR)$(engine_dir)/icons
	install -d $(DESTDIR)$(ibus_dir)/component
	install -d $(DESTDIR)$(app_dir)

	# Install binaries
	install -m 755 target/release/ibus-buffalo  $(DESTDIR)$(lib_dir)/ibus-buffalo
	install -m 755 target/release/buffalo-ui    $(DESTDIR)$(lib_dir)/buffalo-ui

	# Install IBus component XML
	install -m 644 buffalo.xml $(DESTDIR)$(ibus_dir)/component/buffalo.xml

	# Install icons (reuse ibus-bamboo icons for now)
	if [ -d ./icons ]; then \
		cp -R ./icons/* $(DESTDIR)$(engine_dir)/icons/; \
	fi

	# Install desktop entry
	install -m 644 ibus-setup-buffalo.desktop $(DESTDIR)$(app_dir)/ibus-setup-buffalo.desktop

uninstall:
	rm -rf $(DESTDIR)$(lib_dir)
	rm -rf $(DESTDIR)$(engine_dir)
	rm -f  $(DESTDIR)$(ibus_dir)/component/buffalo.xml
	rm -f  $(DESTDIR)$(app_dir)/ibus-setup-buffalo.desktop
