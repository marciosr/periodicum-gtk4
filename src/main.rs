extern crate gtk;
extern crate gio;
extern crate gdk;

#[macro_use]
mod utils;
mod gui;
mod backend;

use crate::gui::*;
use crate::backend::*;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow,Grid, Builder};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/resources"]
struct Asset;

#[derive(RustEmbed)]
#[folder = "data/ui"]
struct Ui;


fn on_activate(application: &gtk::Application) {

	string_from_resource!(glade_src, Ui, "window.ui");

	let builder = Builder::from_string(&glade_src);

	get_widget!(builder, ApplicationWindow, window);
	get_widget!(builder, Grid, grid);

	application.add_window(&window);

	string_from_resource!(dados, Asset, "dados.yaml");
	let estrutura = desserializa(dados);

	build_ui(&grid, estrutura, &window);

	let window_clone = window.clone();
	window.connect_close_request(move |_| {
		window_clone.destroy();
		glib::signal::Inhibit(false)
	});

	for file in Asset::iter() {
      println!("{}", file.as_ref());
  }
	window.show();
}


fn main() {
	let application = Application::new(
		Some("com.github.marciosr.periodicum-gtk4"),
		Default::default(),
	).expect("failed to initialize GTK application");

	application.connect_activate(|app| {

		string_from_resource!(css, Asset, "style.css");

		let provider = gtk::CssProvider::new();
		provider
			.load_from_data(css.as_bytes());

		gtk::StyleContext::add_provider_for_display(
			&gdk::Display::get_default()
			.expect("Error initializing gtk css provider."),
			&provider,
			gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);

		on_activate(app);
	});

	application.run(&std::env::args().collect::<Vec<_>>());

}

