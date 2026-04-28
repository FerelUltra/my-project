pub fn is_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
	if chars.is_empty(){
		return true
	}
    let mut j = chars.len() - 1;
    let mut i = 0;
    while j > i {
        while i < j && chars[i].is_alphanumeric() == false {
            i += 1
        }
        while i < j &&  chars[j].is_alphanumeric() == false {
            j -= 1;
        }

		if i >= j {
            break;
        }
        if chars[i].to_ascii_lowercase() != chars[j].to_ascii_lowercase() {
            return false;
        }
		i += 1;
		j -= 1;
    }
    true
}

fn main(){

}