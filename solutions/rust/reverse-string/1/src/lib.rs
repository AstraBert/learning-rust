pub fn reverse(input: &str) -> String {
    let letters: Vec<char> = input.chars().collect();
    if letters.len() > 0 {
        let mut s = String::new();
        let c = letters.len() - 1;
        let mut d = c as isize;
        while d >= 0 {
            let idx = d as usize;
            s.push(letters[idx]);
            d -= 1;
        }
        return s;
    } else {
        return String::new();
    }
}