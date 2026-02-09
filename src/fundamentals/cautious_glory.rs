// Module factice ajouté pour permettre la compilation des tests.
// Contenu minimal — ne fait rien.
// Retourne les feuilles valides (bool == true) jointes par `;`.
pub fn valid_sheets(data: Vec<(&str, bool)>) -> String {
	let mut parts: Vec<String> = Vec::new();
	for (s, ok) in data {
		if ok {
			parts.push(s.to_string());
		}
	}
	parts.join(";")
}
