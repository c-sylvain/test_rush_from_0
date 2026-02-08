pub mod fundamentals;
pub mod proficiencies;

// Fonction publique implémentée pour l'exercice :
// Elle parcourt un enregistrement de dragons et compte
// combien ont une clé `power` dont la valeur (u32)
// est entre `min_power` et `max_power` inclusivement.
// Règles importantes :
// - Les entrées sont séparées par `;`.
// - Chaque entrée doit être encadrée par `{}` sinon elle est ignorée.
// - À l'intérieur, les paires `key:value` sont séparées par des virgules.
// - On prend uniquement la PREMIÈRE clé `power` rencontrée dans une entrée.
// - Si la valeur n'est pas un entier non signé valide, l'entrée est ignorée.
pub fn dragon_powerscale(record: &str, min_power: u32, max_power: u32) -> usize {
	let mut count: usize = 0;

	for raw_entry in record.split(';') {
		let entry = raw_entry.trim();
		if entry.is_empty() {
			continue;
		}

		// L'entrée doit commencer par '{' et finir par '}'
		if !(entry.starts_with('{') && entry.ends_with('}')) {
			continue; // ignorée si mal formée
		}

		let inner = &entry[1..entry.len()-1];
		let mut found_power = false;

		for pair in inner.split(',') {
			let p = pair.trim();
			if p.is_empty() {
				continue;
			}

			// Une paire valide contient ':' séparant clé et valeur
			let mut parts = p.splitn(2, ':');
			let key_opt = parts.next();
			let val_opt = parts.next();
			if key_opt.is_none() || val_opt.is_none() {
				continue; // paire mal formée -> ignorer
			}
			let key = key_opt.unwrap().trim();
			let val = val_opt.unwrap().trim();

			if key == "power" {
				// On ne considère que la première occurrence de la clé 'power'
				found_power = true;
				// Tenter de parser la valeur en u32
				if let Ok(pw) = val.parse::<u32>() {
					if pw >= min_power && pw <= max_power {
						count += 1;
					}
				}
				break; // sortir de la boucle des paires pour cette entrée
			}
		}

		let _ = found_power; // satisfaction explicite de la règle (pas utilisé)
	}

	count
}


#[cfg(test)]
mod tests {
	use super::dragon_powerscale;

	// Cas nominal (exemple fourni) -> résultat attendu 4
	#[test]
	fn test_nominal() {
		let record = concat!(
			"{name:Alpha,power:5};",
			"{name:Beta,power:7};",
			"{name:Gamma,power:10};",
			"{name:Delta,power:3};",
			"{name:Epsilon,power:not_a_number};",
			"{name:Zeta,power:8,power:100};",
			"{name:NoBrace,power:6"
		);
		assert_eq!(dragon_powerscale(record, 5, 10), 4usize);
	}

	// Entrée sans clé power -> ignorée
	#[test]
	fn test_without_power_ignored() {
		let record = "{name:NoPower,age:100}";
		assert_eq!(dragon_powerscale(record, 0, 1000), 0usize);
	}

	// Valeur power non numérique -> ignorée
	#[test]
	fn test_power_non_numeric_ignored() {
		let record = "{name:Bad,power:abc}";
		assert_eq!(dragon_powerscale(record, 0, 1000), 0usize);
	}

	// Plusieurs clés power -> seule la première compte
	#[test]
	fn test_multiple_power_keys_only_first_counts() {
		let record = "{a:1,power:3,power:100}";
		assert_eq!(dragon_powerscale(record, 0, 10), 1usize);
	}

	// Bornes inclusives min/max
	#[test]
	fn test_bounds_inclusive() {
		let record = "{p:1,power:5};{p:2,power:10};{p:3,power:4}";
		assert_eq!(dragon_powerscale(record, 5, 10), 2usize);
	}

	// Enregistrement vide -> 0
	#[test]
	fn test_empty_record() {
		let record = "";
		assert_eq!(dragon_powerscale(record, 0, 100), 0usize);
	}
}
