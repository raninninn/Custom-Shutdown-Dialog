# Rsh_dlg

## Description

Rsh_dlg is a shutdown dialog for Linux written in Rust and GTK3. Rsh_dlg uses [D-Bus](https://www.freedesktop.org/wiki/Software/dbus/) to shutdown, reboot, suspend or hibernate without needing sudo permissions and by default uses [swaylock](https://github.com/swaywm/swaylock) to lock screens.

## Usage

Invoke Rsh_dlg by executing `rsh_dlg`.

## Installation

    cd rsh_dlg
	cargo build --release
	cp target/release/rsh_dlg SOME_PLACE_IN_YOUR_PATH
