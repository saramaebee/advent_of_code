pub fn p1(_input: Vec<String>) -> String {
	let input: Vec<String> = _input.into_iter().map(|l| l.split("|").map(|k| k.to_string()).collect()).collect();
	let tokens = input.into_iter();

	"Hello World".to_string()
}

pub fn p2(_input: Vec<String>) -> String {
	"Hello World".to_string()
}



// count the occurences of 1 4 7 8 in the string
// 1: len = 2
// 4: len = 4
// 7: len = 3
// 8: len = 7