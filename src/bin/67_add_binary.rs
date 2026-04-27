pub fn add_binary_for_small(a: String, b: String) -> String {
    let a_num: u128 = u128::from_str_radix(&a, 2).unwrap();
    let b_num: u128 = u128::from_str_radix(&b, 2).unwrap();
    format!("{:b}", a_num + b_num)
}

pub fn add_binary(a: String, b: String) -> String {
    let is_a_more_than_b = a.len() > b.len();

    let mut a_bytes = a.into_bytes();
    let mut b_bytes = b.into_bytes();
    let mut carry = 0;
    if is_a_more_than_b {
        for i in 0..b_bytes.len() {
            let sum =
                a_bytes[a_bytes.len() - i - 1] + b_bytes[b_bytes.len() - i - 1] - b'0'*2 + carry;
            carry = sum / 2;
			let idx = a_bytes.len() - i - 1;
            a_bytes[idx] = (sum % 2) + b'0';
        }
		if carry != 0 {
			a_bytes.insert(0, b'1');
		}
        return String::from_utf8(a_bytes).unwrap();
    } else {
        for i in 0..a_bytes.len() {
            let sum =
                a_bytes[a_bytes.len() - i - 1] + b_bytes[b_bytes.len() - i - 1] - b'0'*2 + carry;
            carry = sum / 2;
			let idx = b_bytes.len() - i - 1;
            b_bytes[idx] = (sum % 2) + b'0'
        }
		if carry != 0 {
			b_bytes.insert(0, b'1');
		}
        return String::from_utf8(b_bytes).unwrap();
    }
}

pub fn add_binary_good_solution(a: String, b: String) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut i = a.len() as i32 - 1;
    let mut j = b.len() as i32 - 1;

    let mut carry: u8 = 0;
    let mut result: Vec<u8> = Vec::new();

    while i >= 0 || j >= 0 || carry > 0 {
        let mut sum = carry;

        if i >= 0 {
            sum += a[i as usize] - b'0';
            i -= 1;
        }

        if j >= 0 {
            sum += b[j as usize] - b'0';
            j -= 1;
        }

        result.push((sum % 2) + b'0');
        carry = sum / 2;
    }

    result.reverse();

    String::from_utf8(result).unwrap()
}

fn main() {
    println!("{}", add_binary("101".to_string(), "101010".to_string()));
    let binary = 0b101010;
    println!("{}", binary)
}
