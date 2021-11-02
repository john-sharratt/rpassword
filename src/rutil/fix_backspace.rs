pub fn fix_backspace(mut line: String) -> std::io::Result<String> {

    loop {
        let backspace = match line.chars().position(|c| c == 8 as char) {
            Some(a) => a,
            None => { break; }
        };

        let left = 0usize.max(backspace - 1);
        let right = line.len().min(backspace + 1);
        line = format!("{}{}", &line[..left], &line[right..]);
    }

    Ok(line)
}