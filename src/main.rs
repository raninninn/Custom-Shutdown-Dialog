extern crate gtk;
extern crate gio;
extern crate glib;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;

use gtk::Builder;

use std::env::args;
use std::process::Command;

fn main() {
	let application = gtk::Application::new(
		Some("com.github.ran.rsh_dlg"),
		Default::default(),
	)
	.expect("Init fialed");

	application.connect_activate(|app| {
		build_ui(app);
	});

	application.run(&args().collect::<Vec<_>>());
}

fn build_ui(application: &gtk::Application) {
	let glade_src = include_str!("gtk_surfacev1.glade");
	let builder = Builder::from_string(glade_src);

	let window: gtk::Window = builder.get_object("window1").unwrap();
	window.set_application(Some(application));

	// build UI elements
	let cancel_btn: gtk::Button = builder
		.get_object("cancel_btn")
		.expect("Couldnt get  cancel_btn");
	let reboot_btn: gtk::Button = builder
		.get_object("reboot_btn")
		.expect("couldnt get reboot_btn");
	let shutdown_btn: gtk::Button = builder
		.get_object("shutdown_btn")
		.expect("couldnt get shutdown_btn");
	let suspend_btn: gtk::Button = builder
		.get_object("suspend_btn")
		.expect("couldnt get suspend_btn");
	let hibernate_btn: gtk::Button = builder
		.get_object("hibernate_btn")
		.expect("couldnt get hibernate_btn");
	let lock_btn: gtk::Button = builder
		.get_object("lock_btn")
		.expect("couldnt get lock_btn");
	let conf_dialog: gtk::Dialog = builder
		.get_object("conf_dialog")
		.expect("couldnt get conf_dialog");
	let dialog_ok_btn: gtk::Button = builder
		.get_object("dialog_ok_btn")
		.expect("couldnt get dialog_ok_btn");
	let dialog_cancel_btn: gtk::Button = builder
		.get_object("dialog_cancel_btn")
		.expect("couldnt get dialog_cancel_btn");

	// Clone confimation dialog and ok button for reboot function
	let dialog_ok_btn_reboot = dialog_ok_btn.clone();
	let conf_dialog_reboot = conf_dialog.clone();

	// Set ´keep_above´ for conf_dialogs
	window.set_keep_above(true);
	conf_dialog.set_keep_above(true);
	conf_dialog_reboot.set_keep_above(true);

	// Connect buttons
	cancel_btn.connect_clicked(clone!(@weak window => move |_| {
		std::process::exit(0);
	}));
	dialog_cancel_btn.connect_clicked(clone!(@weak window => move |_| {
		std::process::exit(0);
	}));

	shutdown_btn.connect_clicked(clone!(@weak window => move |_| {
		conf_dialog.show_all();
		dialog_ok_btn.connect_clicked(clone!(@weak window => move |_| {
			Command::new("dbus-send")
				.arg("--print-reply").arg("--system")
				.arg("--dest=org.freedesktop.login1")
				.arg("/org/freedesktop/login1")
				.arg("org.freedesktop.login1.Manager.PowerOff")
				.arg("boolean:true")
				.output().expect("Shutting down failed");
		}));
	}));
	reboot_btn.connect_clicked(clone!(@weak window => move |_| {
		conf_dialog_reboot.show_all();
		dialog_ok_btn_reboot.connect_clicked(clone!(@weak window => move |_| {
			Command::new("dbus-send")
				.arg("--print-reply").arg("--system")
				.arg("--dest=org.freedesktop.login1")
				.arg("/org/freedesktop/login1")
				.arg("org.freedesktop.login1.Manager.Reboot")
				.arg("boolean:true")
				.output().expect("Rebooting failed");

		}));
	}));
	suspend_btn.connect_clicked(clone!(@weak window => move |_| {
		Command::new("dbus-send")
			.arg("--print-reply").arg("--system")
			.arg("--dest=org.freedesktop.login1")
			.arg("/org/freedesktop/login1")
			.arg("org.freedesktop.login1.Manager.Suspend")
			.arg("boolean:true")
			.output().expect("Suspending failed");
	}));
	hibernate_btn.connect_clicked(clone!(@weak window => move |_| {
		Command::new("dbus-send")
			.arg("--print-reply").arg("--system")
			.arg("--dest=org.freedesktop.login1")
			.arg("/org/freedesktop/login1")
			.arg("org.freedesktop.login1.Manager.Hibernate")
			.arg("boolean:true")
			.output().expect("hibernating failed");
	}));
	lock_btn.connect_clicked(clone!(@weak window => move |_| {
		Command::new("swaylock")
		.output().expect("Locking failed");
	}));

	window.show_all();
}
