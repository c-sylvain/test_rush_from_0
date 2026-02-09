// Fonction qui réalise une transformation 'maudite' :
// on inverse les séquences de lettres et on repositionne les caractères
// non-lettres en gardant leur ordre relatif.
pub fn cursed_reverse(s: &mut String) {
	// Extraire indices des séquences de lettres et inverser chaque séquence
	let mut chars: Vec<char> = s.chars().collect();
	let n = chars.len();
	let mut i = 0usize;
	while i < n {
		if chars[i].is_alphabetic() {
			let start = i;
			while i < n && chars[i].is_alphabetic() { i += 1; }
			let end = i; // exclusive
			chars[start..end].reverse();
		} else {
			i += 1;
		}
	}
	// Reconstruire la chaîne
	*s = chars.into_iter().collect();
}
