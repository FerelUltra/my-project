fn main(){

}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
	let mut digits = digits;
	let mut carry = true;
	for digit in digits.iter_mut().rev(){
		*digit = *digit + 1;
		if *digit == 10{
			*digit = 0
		} else {
			carry = false;
			break;
		}

	}

	if carry{
		digits.insert(0,1);
	}

	digits
}