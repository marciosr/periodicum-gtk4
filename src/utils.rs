macro_rules! get_widget {
	($builder:expr, $wtype:ty, $name:ident) => {
		let $name: $wtype = $builder.get_object(stringify!($name))
			.expect(&format!("Could not find widget \"{}\"", stringify!($name)));
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

		boxe.append(&lb_an);
		boxe.append(&lb_sy);
		boxe.append(&lb_aw);

		$name.set_child(Some(&boxe));

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
// O uso da macro é: string_from_resource( variaval,
// 																				 estrutura dos recursos,
// 																				 "nome do arquivo")
macro_rules! string_from_resource {
	($string_var_name:ident, $resource:ident, $file_name:expr) => {

	let $string_var_name: String = String::from(
		std::str::from_utf8(
		$resource::get($file_name)
		.unwrap()
		.as_ref())
		.unwrap());
	};
}
