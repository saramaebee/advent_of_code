pub fn p1(_input: Vec<String>) -> String {

	let mut error_scores: Vec<usize> = Vec::new();

	for line in _input {
		let input: Vec<String> = line.split("").filter(|l| l.len() > 0).map(|k| k.to_string()).collect::<Vec<String>>().into_iter().collect();
		let mut stack: Vec<String> = Vec::new();
		let mut err_stack: Vec<Option<String>> = Vec::new();

		for token in input {
			if is_open_bracket(&token) {
				stack.push((&token).to_string());
			} else {
				let x = if stack.len() > 0 { (&stack[stack.len() - 1]).to_string() } else { get_opening_bracket(&token) };
				if x.to_string() != get_opening_bracket(&token) {
					err_stack.push(Some(token));
				}
				stack.pop();
			}
		}

		if err_stack.len() > 0 {
			if let Some(first_err) = &err_stack[&err_stack.len() - 1] {
				let mut score = 0usize;
				match first_err as &str {
					")" => {score = 3usize},
					"]" => {score = 57usize},
					"}" => {score = 1197usize},
					">" => {score = 25137usize},
					_ => {},
				}
				error_scores.push(score)
			}
		}
	}

	error_scores.into_iter().sum::<usize>().to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	let mut completion_scores: Vec<usize> = Vec::new();

	for line in _input {
		let input: Vec<String> = line.split("").filter(|l| l.len() > 0).map(|k| k.to_string()).collect::<Vec<String>>().into_iter().collect();
		let mut stack: Vec<String> = Vec::new();
		let mut err_stack: Vec<Option<String>> = Vec::new();
		let mut score: usize = 0;

		for token in input {
			if is_open_bracket(&token) {
				stack.push((&token).to_string());
			} else {
				let x = if stack.len() > 0 { (&stack[stack.len() - 1]).to_string() } else { get_opening_bracket(&token) };
				if x.to_string() != get_opening_bracket(&token) {
					err_stack.push(Some(token));
				}
				stack.pop();
			}
		}

		let mut closing_string: Vec<String> = Vec::new();

		if err_stack.len() == 0 {
			while let Some(opening_bracket) = stack.pop() {
				closing_string.push(get_closing_bracket(&opening_bracket));
			}

			if closing_string.len() > 0 {
				score = get_completion_score(closing_string);
			}
			completion_scores.push(score)
		}

	}
	completion_scores.sort();
	completion_scores[completion_scores.len() / 2].to_string()

}

fn is_open_bracket(token: &str) -> bool {
		token == "<" ||
		token == "{" ||
		token == "[" ||
		token == "("
}

fn get_opening_bracket(token: &str) -> String {
	match token {
		")" => "(".to_string(),
		"]" => "[".to_string(),
		"}" => "{".to_string(),
		">" => "<".to_string(),
		_ => "ERR".to_string(),
	}
}

fn get_closing_bracket(token: &str) -> String {
	match token {
		"{" => "}".to_string(),
		"[" => "]".to_string(),
		"(" => ")".to_string(),
		"<" => ">".to_string(),
		_ => "ERR".to_string()
	}
}

fn get_completion_score(completion_string_vec: Vec<String>) -> usize{
	let mut score = 0;
	for token in completion_string_vec {
		score *= 5;
		match token.as_str() {
			")" => { score += 1},
			"]" => { score += 2},
			"}" => { score += 3 },
			">" => { score += 4 },
			_ => {},
		}
	}

	score
}