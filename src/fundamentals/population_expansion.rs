// Parse un enregistrement de la forme `key:val, key:val` et formate
// la chaîne comme demandé dans les tests.
pub fn register_newcomer(record: &str) -> String {
	let mut name = "".to_string();
	let mut power = "".to_string();
	let mut age = "".to_string();

	for pair in record.split(',') {
		let p = pair.trim();
		if p.is_empty() { continue; }
		let mut parts = p.splitn(2, ':');
		if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
			let key = k.trim().to_lowercase();
			let val = v.trim();
			match key.as_str() {
				"name" => name = val.to_string(),
				"power" => power = val.to_string(),
				"age" => age = val.to_string(),
				_ => {}
			}
		}
	}

	format!("{} | POWER [{}] | AGE [{}] YEARS", name, power, age)
}
