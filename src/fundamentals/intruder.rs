// Vérifie l'identité : recherche dans la liste la ligne correspondant au nom.
// Si une ligne contient le troisième champ `correct`, elle est privilégiée.
pub fn verify_identity(name: &str, list: &str) -> String {
	let mut fallback: Option<String> = None;
	for line in list.lines() {
		let parts: Vec<&str> = line.split('|').collect();
		if parts.len() >= 2 && parts[1] == name {
			if parts.len() >= 3 && parts[2] == "correct" {
				return line.to_string();
			}
			if fallback.is_none() {
				fallback = Some(line.to_string());
			}
		}
	}
	fallback.unwrap_or_else(|| "".to_string())
}
