fn main() {

	let examples = vec![
        vec![30, 40, 35, 50],
        vec![73, 74, 75, 71, 69, 72, 76, 73],
        vec![30, 30, 30],
        vec![30, 29, 28],
        vec![28, 29, 30],
    ];

    for input in examples {
        let result = daily_temperatures_right(input.clone());

        println!("input:  {:?}", input);
        println!("result: {:?}", result);
        println!();
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        if (i != 0 && chars[i] == '}' && stack.last() == Some(&'{'))
            || (i != 0 && chars[i] == ']' && stack.last() == Some(&'['))
            || (i != 0 && chars[i] == ')' && stack.last() == Some(&'('))
        {
            stack.pop();
        } else {
            stack.push(chars[i]);
        }
    }

    stack.is_empty()
}

pub fn daily_temperatures(v: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; v.len()];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..v.len() {
        while let Some(&last_i) = stack.last() {
            if v[i] > v[last_i] {
                stack.pop();
                result[last_i] = (i - last_i) as i32;
            } else {
                break;
            }
        }

        stack.push(i);
    }

    result
}