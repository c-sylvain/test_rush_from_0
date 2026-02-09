// Implémentation de dragon_powerscale conformément aux consignes.
// Cette fonction analyse un enregistrement et compte les entrées
// dont la clé `power` (première occurrence) est un u32 valide
// et se trouve entre `min_power` et `max_power` inclus.
pub fn dragon_powerscale(record: &str, min_power: u32, max_power: u32) -> usize {
	let mut count: usize = 0;

	for raw_entry in record.split(';') {
		let entry = raw_entry.trim();
		if entry.is_empty() {
			continue;
		}

		if !(entry.starts_with('{') && entry.ends_with('}')) {
			continue;
		}

		let inner = &entry[1..entry.len()-1];
		for pair in inner.split(',') {
			let p = pair.trim();
			if p.is_empty() {
				continue;
			}
			let mut parts = p.splitn(2, ':');
			let key_opt = parts.next();
			let val_opt = parts.next();
			if key_opt.is_none() || val_opt.is_none() {
				continue;
			}
			let key = key_opt.unwrap().trim();
			let val = val_opt.unwrap().trim();
			if key == "power" {
				if let Ok(pw) = val.parse::<u32>() {
					if pw >= min_power && pw <= max_power {
						count += 1;
					}
				}
				break;
			}
		}
	}

	count
}
