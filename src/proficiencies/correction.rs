// Recherche les mots bannis hors des crochets `[...]` et effectue
// un remplacement simple. Retourne (liste_des_mots_trouves, texte_corrigé)
pub fn find_banned(text: &str, banned: &Vec<(&str, &str)>) -> (Vec<String>, String) {
	let mut found: Vec<String> = Vec::new();
	let mut out = String::new();
	let mut i = 0usize;
	let chars: Vec<char> = text.chars().collect();
	let n = chars.len();
	let mut in_brackets = false;

	while i < n {
		let c = chars[i];
		if c == '[' {
			in_brackets = true;
			out.push(c);
			i += 1;
			continue;
		}
		if c == ']' {
			in_brackets = false;
			out.push(c);
			i += 1;
			continue;
		}

		if in_brackets {
			out.push(c);
			i += 1;
			continue;
		}

		// hors crochets : essayer de détecter une occurrence bannie
		let mut matched = false;
		for (key, repl) in banned.iter() {
			let keylen = key.len();
			if i + keylen <= n {
				let slice: String = chars[i..i+keylen].iter().collect();
				if slice.eq_ignore_ascii_case(*key) {
					// étendre à token complet (lettres et '-')
					let mut l = i;
					while l > 0 && (chars[l-1].is_alphanumeric() || chars[l-1] == '-') { l -= 1; }
					let mut r = i + keylen;
					while r < n && (chars[r].is_alphanumeric() || chars[r] == '-') { r += 1; }
					let token: String = chars[l..r].iter().collect();
					found.push(token.clone());

					// construire remplacement pour la portion [l..r]
					// on va remplacer uniquement la portion du mot correspondant à la clé
					// pour simplifier, on remplace slice i..i+keylen par repl
					out.push_str(&chars[l..i].iter().collect::<String>());
					out.push_str(repl);
					out.push_str(&chars[i+keylen..r].iter().collect::<String>());
					i = r;
					matched = true;
					break;
				}
			}
		}
		if !matched {
			out.push(c);
			i += 1;
		}
	}

	(found, out)
}
