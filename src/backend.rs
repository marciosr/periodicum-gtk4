use std::fs::File;
use std::io::prelude::*;
use std::fs;
use serde::{Serialize, Deserialize};

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


#[allow(dead_code)]
pub fn serializa_yaml (vec: &Vec::<ElementData>) -> String {
	let serializado = serde_yaml::to_string(&vec).unwrap();
	serializado
}

pub fn desserializa (serializado: String) -> Vec::<ElementData> {
	let desserializado: Vec::<ElementData> = serde_yaml::from_str(&serializado).unwrap();
	desserializado
}

pub fn carrega_dados () -> std::io::Result<String> {
	let file = fs::read_to_string("dados.yaml")?;
	Ok(file)
}

#[allow(dead_code)]
pub fn salva_dados (serializado: String,
								file_name: &str) -> std::io::Result<()> {
	let mut file = File::create(file_name)?;
	file.write_all(&serializado.as_bytes())?;
	println!("Perfis salvos no disco.");

	Ok(())
}

