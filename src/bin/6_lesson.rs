fn main(){

}

pub fn is_valid(s: String) -> bool{
	let mut stack: Vec<char> = Vec::new();
	let chars: Vec<char> = s.chars().collect();
	for i in 0..chars.len(){
		if (i != 0 && chars[i] == '}' && stack.last() == Some(&'{')) ||
		(i != 0 &&  chars[i] == ']' && stack.last() == Some(&'[')) ||
		(i != 0 &&  chars[i] == ')' && stack.last() == Some(&'('))
		{
			stack.pop();
		} else{
			stack.push(chars[i]);
		}
	}

	stack.is_empty()
}