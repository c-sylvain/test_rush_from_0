// Décodage d'un message avec répétitions du type `3*word` ou `2*(a b)`.
pub fn decode_message(input: &str) -> String {
	// Parser récursif simple pour groupes parenthésés et répétitions
	fn parse_seq(s: &str, i: &mut usize) -> Vec<String> {
		let mut out: Vec<String> = Vec::new();
		let chars: Vec<char> = s.chars().collect();
		let n = chars.len();
		while *i < n {
			// sauter les espaces
			while *i < n && chars[*i].is_whitespace() { *i += 1; }
			if *i >= n { break; }
			if chars[*i] == ')' { break; }

			// lire un nombre éventuel
			let start = *i;
			while *i < n && chars[*i].is_digit(10) { *i += 1; }
			let mut repeat = None;
			if *i > start && *i < n && chars[*i] == '*' {
				// nombre trouvé
				let num: usize = s[start..*i].parse().unwrap_or(1);
				*i += 1; // sauter '*'
				repeat = Some(num);
			}

			if *i < n && chars[*i] == '(' {
				*i += 1; // sauter '('
				let group = parse_seq(s, i);
				if *i < n && chars[*i] == ')' { *i += 1; }
				// concatène l'expansion du groupe
				let expanded = group.join(" ");
				if let Some(r) = repeat {
					let mut acc = String::new();
					for _ in 0..r { acc.push_str(&expanded); }
					out.push(acc);
				} else {
					out.push(expanded);
				}
			} else {
				// lire un mot (jusqu'à espace ou ')' )
				let wstart = *i;
				while *i < n && !chars[*i].is_whitespace() && chars[*i] != ')' { *i += 1; }
				let word = s[wstart..*i].to_string();
				if let Some(r) = repeat {
					let mut acc = String::new();
					for _ in 0..r { acc.push_str(&word); }
					out.push(acc);
				} else {
					out.push(word);
				}
			}
		}
		out
	}

	let mut idx = 0usize;
	let parts = parse_seq(input, &mut idx);
	parts.join(" ")
}
