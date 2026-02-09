// Évalue des expressions simples contenant des opérateurs + - * %
// Les tokens sont séparés par des espaces (conforme aux tests).
pub fn compute(expr: &str) -> i64 {
	let mut tokens: Vec<&str> = expr.split_whitespace().collect();

	// première passe : gérer * et %
	let mut stack: Vec<String> = Vec::new();
	let mut i = 0;
	while i < tokens.len() {
		let tk = tokens[i];
		if tk == "*" || tk == "%" {
			let left = stack.pop().unwrap().parse::<i64>().unwrap();
			let right = tokens[i+1].parse::<i64>().unwrap();
			let res = if tk == "*" { left * right } else { left % right };
			stack.push(res.to_string());
			i += 2;
		} else {
			stack.push(tk.to_string());
			i += 1;
		}
	}

	// seconde passe : + et -
	let mut acc = stack[0].parse::<i64>().unwrap();
	let mut j = 1;
	while j < stack.len() {
		let op = stack[j].as_str();
		let val = stack[j+1].parse::<i64>().unwrap();
		if op == "+" { acc += val; } else { acc -= val; }
		j += 2;
	}
	acc
}
