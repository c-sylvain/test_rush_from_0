// Analyse une feuille et retourne (content, byte_len, char_len, char_len/token_count)
pub fn swords_analysis(s: &str) -> (String, usize, usize, f64) {
	let content = s.to_string();
	let byte_len = content.as_bytes().len();
	let char_len = content.chars().count();
	let token_count = content.split_whitespace().count().max(1);
	let ratio = char_len as f64 / token_count as f64;
	(content, byte_len, char_len, ratio)
}
