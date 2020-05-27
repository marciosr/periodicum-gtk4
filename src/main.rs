extern crate gtk;
extern crate gio;
extern crate gdk;
// use std::fs::File;
use std::{fs, fs::File};
use std::io::prelude::*;
use std::rc::Rc;
use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;
use serde::{Serialize, Deserialize};

use gtk::{Application, ApplicationWindow, Button, Grid, Builder, Label, HeaderBar, Window, Box};

macro_rules! get_widget {
	($builder:expr, $wtype:ty, $name:ident) => {
		let $name: $wtype = $builder.get_object(stringify!($name)).expect(&format!("Could not find widget \"{}\"", stringify!($name)));
	};
}

macro_rules! build_widget {
	($name:ident, $wtype:ident, $vetor:ident, $window:ident) => {

		let $name: gtk::$wtype = gtk::$wtype::new();

		let emto = $vetor.clone();
		let window_clone = $window.clone(); // Compila, mas não cria os botões.

		let lb_sy: gtk::Label = gtk::Label::new(Some(&emto.symbol));
		gtk::prelude::WidgetExtManual::set_name(&lb_sy,"symbol");
		let lb_an: gtk::Label = gtk::Label::new(Some(&emto.atomic_number.to_string()));
		gtk::prelude::WidgetExtManual::set_name(&lb_an,"atomic_number");
		let lb_aw: gtk::Label = gtk::Label::new(Some(&emto.atomic_weigth));
		gtk::prelude::WidgetExtManual::set_name(&lb_aw,"atomic_weight");
		let boxe: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 1);

		boxe.add(&lb_an);
		boxe.add(&lb_sy);
		boxe.add(&lb_aw);

		$name.add(&boxe);

		match emto.atomic_number {
			1 | 6 | 7 | 8 | 15 | 16 | 34 => gtk::prelude::WidgetExtManual::set_name(&$name,"nm"),
			3 | 11 | 19 | 37 | 55 | 87 => gtk::prelude::WidgetExtManual::set_name(&$name,"ma"),
			4 | 12 | 20 | 38 | 56 | 88 => gtk::prelude::WidgetExtManual::set_name(&$name,"mat"),
			21..=30 | 39..=48 | 72..=80 | 104..=112  => gtk::prelude::WidgetExtManual::set_name(&$name,"mt"),
			5 | 14 | 32 | 33 | 51 | 52 | 84 => gtk::prelude::WidgetExtManual::set_name(&$name,"sm"),
			13 | 31 | 49 | 50 | 81 | 82 | 83 | 113 | 114 | 115 | 116 => gtk::prelude::WidgetExtManual::set_name(&$name,"om"),
			9 | 17 | 35 | 53 | 85 | 117 => gtk::prelude::WidgetExtManual::set_name(&$name,"hg"),
			2 | 10 | 18 | 36 | 54 | 86 | 118 => gtk::prelude::WidgetExtManual::set_name(&$name,"gn"),
			57..=71 => gtk::prelude::WidgetExtManual::set_name(&$name,"lt"),
			89..=103 => gtk::prelude::WidgetExtManual::set_name(&$name,"ac"),
			_ => println!("Erro, grupo inexistente"),
		}

		$name.connect_clicked (move|_| {
			println!("O botão {}", emto.name);
			let dialogo = ElementDialog::new();
			dialogo.run(&emto, &window_clone);
		});
	};
}

macro_rules! wid {
	($nome_button:expr) => {
		$nome_button
	};
}

const STYLE: &str = "
/* Aplicável a todos os botões */
button {
				border-radius: 0;
        border-width: 0;
        background-image: none;
}

/* Não metais	 */
button#nm {
        background-image: none;
        background-color: #aadc59;
}

button#nm:hover {
        background-color: #73953e;
}

/* Metais alcalinos */
button#ma {
        background-image: none;
        background-color: #f3c556;
}

button#ma:hover {
        background-color: #9e7f34;
}

/* Metais alcalino-terrosos */
button#mat {
        background-image: none;
        background-color: #ebe753;
}

button#mat:hover {
        background-color: #989638;
}
/* Metais de transição */
button#mt {
        background-image: none;
        background-color: #f3a0ab;
}

button#mt:hover {
        background-color: #b2767e;
}

/* Semimetais */
button#sm {
        background-image: none;
        background-color: #78c9b9;
}

button#sm:hover {
        background-color: #5d9d90;
}

/* Outros metais */
button#om {
        background-image: none;
        background-color: #a4cbd3;
}

button#om:hover {
        background-color: #799499;
}

/* Halogênios */
button#hg {
        background-image: none;
        background-color: #b2e4f5;
}

button#hg:hover {
        background-color: #7fa7b5;
}

/* Gases nobres */
button#gn {
        background-image: none;
        background-color: #79b1e2;
}

button#gn:hover {
        background-color: #577fa3;
}

/* Lantanídeos */
button#lt {
        background-image: none;
        background-color: #9ddfe0;
}

button#lt:hover {
        background-color: #7cb1b2;
}

/* Actinídios */
button#ac {
        background-image: none;
        background-color: #e5b3dd;
}

button#ac:hover {
        background-color: #a17e9b;
}

label#symbol {
	color: #000000;
	font-size: 100%;
}

label#atomic_number {
	color: #000000;
	font-size: 60%;
}

label#atomic_weight {
	color: #000000;
	font-size: 60%;
}";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementData {
	pub name:												String,
	pub symbol:											String,
	pub atomic_number:							i32,
	pub atomic_weigth:							String,
	pub density:										String,
	pub melting_point:							String,
	pub boiling_point:							String,
	pub atomic_radius:							String,
	pub covalent_radius:						String,
	pub ionic_radius:								String,
	pub atomic_volume:							String,
	pub specific_heat:							String,
	pub fusion_heat:								String,
	pub evaporation_heat:						String,
	pub thermal_conductivity:				String,
	pub debye_temperature:					String,
	pub pauling_negativity_number:	String,
	pub first_ionizing_energy:			String,
	pub oxidation_states:						String,
	pub eletronic_configuration:		String,
	pub lattice_structure:					String,
	pub lattice_constant:						String,
	pub lattice_ca_ratio:						String,
	pub appearance:									String,
	pub discovery_date:							String,
	pub discovered_by:							String,
	pub named_after:								String,
	pub pos_x:											i32,
	pub pos_y:											i32,
}

pub struct ElementDialog {
	pub dialog:											Window,
	pub headerbar:									HeaderBar,
	pub name:												Label,
	pub symbol:											Label,
	pub atomic_number:							Label,
	pub atomic_weigth:							Label,
	pub density:										Label,
	pub melting_point:							Label,
	pub boiling_point:							Label,
	pub atomic_radius:							Label,
	pub covalent_radius:						Label,
	pub ionic_radius:								Label,
	pub atomic_volume:							Label,
	pub specific_heat:							Label,
	pub fusion_heat:								Label,
	pub evaporation_heat:						Label,
	pub thermal_conductivity:				Label,
	pub debye_temperature:					Label,
	pub pauling_negativity_number:	Label,
	pub first_ionizing_energy:			Label,
	pub oxidation_states:						Label,
	pub eletronic_configuration:		Label,
	pub lattice_structure:					Label,
	pub lattice_constant:						Label,
	pub lattice_ca_ratio:						Label,
	pub appearance:									Label,
	pub discovery_date:							Label,
	pub discovered_by:							Label,
	pub named_after:								Label,
	pub bt_close:										Button,
}

impl ElementDialog {
	pub fn new() -> Rc<Self> {

		let glade_src = include_str!("dialog.ui");
		let builder = Builder::new_from_string(glade_src);

		get_widget!(builder, Window, dialog);
		get_widget!(builder, HeaderBar, headerbar);
		get_widget!(builder, Label, name);
		get_widget!(builder, Label, symbol);
		get_widget!(builder, Label, atomic_number);
		get_widget!(builder, Label, atomic_weigth);
		get_widget!(builder, Label, density);
		get_widget!(builder, Label, melting_point);
		get_widget!(builder, Label, boiling_point);
		get_widget!(builder, Label, atomic_radius);
		get_widget!(builder, Label, covalent_radius);
		get_widget!(builder, Label, ionic_radius);
		get_widget!(builder, Label, atomic_volume);
		get_widget!(builder, Label, specific_heat);
		get_widget!(builder, Label, fusion_heat);
		get_widget!(builder, Label, evaporation_heat);
		get_widget!(builder, Label, thermal_conductivity);
		get_widget!(builder, Label, debye_temperature);
		get_widget!(builder, Label, pauling_negativity_number);
		get_widget!(builder, Label, first_ionizing_energy);
		get_widget!(builder, Label, oxidation_states);
		get_widget!(builder, Label, eletronic_configuration);
		get_widget!(builder, Label, lattice_structure);
		get_widget!(builder, Label, lattice_constant);
		get_widget!(builder, Label, lattice_ca_ratio);
		get_widget!(builder, Label, appearance);
		get_widget!(builder, Label, discovery_date);
		get_widget!(builder, Label, discovered_by);
		get_widget!(builder, Label, named_after);
		get_widget!(builder, Button, bt_close);

		let dialog_clone = dialog.clone();
		dialog.connect_close_request(move |_|{
			dialog_clone.remove(&dialog_clone);
			glib::signal::Inhibit(false)
		});

		bt_close.connect_clicked(clone!(@strong dialog => move |_| {
			dialog.destroy();
		}));

		let element_dialog = Rc::new(Self {	dialog, headerbar, name, symbol, atomic_number, atomic_weigth, density,
																melting_point, boiling_point, atomic_radius, covalent_radius, ionic_radius,
																atomic_volume, specific_heat, fusion_heat, evaporation_heat, thermal_conductivity,
																debye_temperature, pauling_negativity_number, first_ionizing_energy, oxidation_states,
																eletronic_configuration, lattice_structure, lattice_constant, lattice_ca_ratio,
																appearance, discovery_date, discovered_by, named_after, bt_close });
		element_dialog
	}
// Aqui não será necessário passar o vetor com os dados de todo os elementos,
// apenas o item referente ao elemento atual vec[i]. É preciso ver se há
// métodos especiais para fazer isso, ou se vai direto.
	pub fn run (&self, element: &ElementData, window: &gtk::ApplicationWindow) {

		println!("Dentro do run da implementação");
		self.dialog.set_transient_for(Some(window));

		self.name.set_label(element.name.as_str());
		self.symbol.set_label(element.symbol.as_str());
		self.atomic_number.set_label(&element.atomic_number.to_string());
		self.atomic_weigth.set_label(element.atomic_weigth.as_str());
		self.density.set_label(element.density.as_str());
		self.melting_point.set_label(element.melting_point.as_str());
		self.boiling_point.set_label(element.boiling_point.as_str());
		self.atomic_radius.set_label(element.atomic_radius.as_str());
		self.covalent_radius.set_label(element.covalent_radius.as_str());
		self.ionic_radius.set_label(element.ionic_radius.as_str());
		self.atomic_volume.set_label(element.atomic_volume.as_str());
		self.specific_heat.set_label(element.specific_heat.as_str());
		self.fusion_heat.set_label(element.fusion_heat.as_str());
		self.evaporation_heat.set_label(element.evaporation_heat.as_str());
		self.thermal_conductivity.set_label(element.thermal_conductivity.as_str());
		self.debye_temperature.set_label(element.debye_temperature.as_str());
		self.pauling_negativity_number.set_label(element.pauling_negativity_number.as_str());
		self.first_ionizing_energy.set_label(element.first_ionizing_energy.as_str());
		self.oxidation_states.set_label(element.oxidation_states.as_str());
		self.eletronic_configuration.set_label(element.eletronic_configuration.as_str());
		self.lattice_structure.set_label(element.lattice_structure.as_str());
		self.lattice_constant.set_label(element.lattice_constant.as_str());
		self.lattice_ca_ratio.set_label(element.lattice_ca_ratio.as_str());
		self.appearance.set_label(element.appearance.as_str());
		self.discovery_date.set_label(element.discovery_date.as_str());
		self.discovered_by.set_label(element.discovered_by.as_str());
		self.named_after.set_label(element.named_after.as_str());
		self.headerbar.set_title(Some(element.name.as_str()));

		self.dialog.show();
	}
}


fn on_activate(application: &gtk::Application) {

	let glade_src = include_str!("window.ui");
	let builder = Builder::new_from_string(glade_src);

	get_widget!(builder, ApplicationWindow, window);
	get_widget!(builder, Grid, grid);

	application.add_window(&window);

	// Cria a tabela propriamente dita
	let estrutura = match carrega_dados() {
		Ok(serializado) => desserializa(serializado),
		Err(_e) => panic!("Arquivo de dados não encontrado!"),
	};


	build_ui(&grid, estrutura, &window);
	window.show();
}


fn main() {
	let application = Application::new(
		Some("com.github.marciosr.periodicum-gtk4"),
		Default::default(),
	).expect("failed to initialize GTK application");

	application.connect_activate(|app| {

	// let provider = gtk::CssProvider::new();
	// provider
	// 	.load_from_data(STYLE.as_bytes());
		//.expect("Failed to load CSS");

	// gtk::StyleContext::add_provider_for_display(
	// 	&gdk::Display::get_default().expect("Error initializing gtk css provider."),
	// 	&provider,
	// 	gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	// );


	on_activate(app);
	});

	application.run(&[]);
}


#[allow(dead_code)]
fn serializa_yaml (vec: &Vec::<ElementData>) -> String {
	let serializado = serde_yaml::to_string(&vec).unwrap();
	serializado
}

fn desserializa (serializado: String) -> Vec::<ElementData> {
	let desserializado: Vec::<ElementData> = serde_yaml::from_str(&serializado).unwrap();
	desserializado
}

pub fn carrega_dados () -> std::io::Result<String> {
	let file = fs::read_to_string("dados.yaml")?;
	Ok(file)
}

#[allow(dead_code)]
fn salva_dados (serializado: String, file_name: &str) -> std::io::Result<()> {
	let mut file = File::create(file_name)?;
	file.write_all(&serializado.as_bytes())?;
	println!("Perfis salvos no disco.");

	Ok(())
}


fn build_ui (grid: &gtk::Grid, vec: Vec<ElementData>, window: &gtk::ApplicationWindow) {

	for i in 0..vec.len() {
		println!("Elemento {} coluna {} linha {}", vec[i].name, vec[i].pos_x, vec[i].pos_y);

		let elemento: &ElementData = &vec[i];

		let col = vec[i].pos_x;
		let lin = vec[i].pos_y;
		let bt_name = format!("bt_{}", i);

		build_widget!(bt_name, Button, elemento, window);

		grid.attach(&wid!(bt_name), col, lin, 1, 1);
	}
}
