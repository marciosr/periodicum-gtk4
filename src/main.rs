extern crate gtk;
extern crate gio;
extern crate gdk;

#[macro_use]
mod utils;
mod gui;
mod backend;
mod table;

use crate::table::*;
use crate::gui::*;
use crate::backend::*;

use gtk::prelude::*;
use gio::prelude::*;


use gtk::{Application, ApplicationWindow,Grid, Builder};


fn on_activate(application: &gtk::Application) {

	let glade_src = include_str!("window.ui");
	let builder = Builder::from_string(glade_src);

	get_widget!(builder, ApplicationWindow, window);
	get_widget!(builder, Grid, grid);

	application.add_window(&window);

	// Cria a tabela propriamente dita
	let estrutura = match carrega_dados() {
		Ok(serializado) => desserializa(serializado),
		Err(_e) => desserializa(make_table().to_string()),
	};


	build_ui(&grid, estrutura, &window);

	let window_clone = window.clone();
	window.connect_close_request(move |_| {
		window_clone.destroy();
		glib::signal::Inhibit(true)
	});
	window.show();
}


fn main() {
	let application = Application::new(
		Some("com.github.marciosr.periodicum-gtk4"),
		Default::default(),
	).expect("failed to initialize GTK application");

	application.connect_activate(|app| {

	let provider = gtk::CssProvider::new();
	provider
		.load_from_data(STYLE.as_bytes());
		//.expect("Failed to load CSS");

	gtk::StyleContext::add_provider_for_display(
		&gdk::Display::get_default().expect("Error initializing gtk css provider."),
		&provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);


	on_activate(app);
	});

	application.run(&[]);
	// let app_clone = application.clone();
	// application.connect_shutdown(move |_|{
	// 	app_clone.destroy();
	// });
}

