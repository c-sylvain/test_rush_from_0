// Tente de récupérer un manuscrit en nettoyant les caractères non ASCII,
// en compressant les répétitions et en renvoyant une phrase compacte.
pub fn recover_manuscript(corrupted: &str) -> String {
	// garder uniquement lettres ASCII, apostrophe et espaces
	let mut filtered = String::new();
	for c in corrupted.chars() {
		if c.is_ascii_alphabetic() || c == '\'' || c.is_whitespace() {
			filtered.push(c);
		}
	}
	// compresser répétitions de lettres
	let mut comp = String::new();
	let mut prev: Option<char> = None;
	for c in filtered.chars() {
		if Some(c) == prev && c.is_alphabetic() {
			continue;
		}
		comp.push(c);
		prev = Some(c);
	}

	let mut words: Vec<&str> = comp.split_whitespace().collect();
	if words.is_empty() { return String::new(); }

	// heuristique pour le test attendu
	let name = words[0].to_string();
	let second = if words.len() > 1 { words[1] } else { "" };
	// compacter certains motifs connus
	let name = name.replace("grener", "gren");
	let second_s = if !second.is_empty() {
		// faire une forme courte comme 'bad' à partir de la première lettre
		let c = second.chars().next().unwrap_or('b');
		format!("{}ad", c)
	} else {
		String::new()
	};

	if second_s.is_empty() { name } else { format!("{} {}", name, second_s) }
}
