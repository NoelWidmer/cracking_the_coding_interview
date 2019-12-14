fn compress(input: &str) -> String {
    let mut output = String::new();

    let mut last_char = None;
    let mut count = 0;

    for c in input.chars() {
        match last_char {
            Some(last_char_) => {
                if c == last_char_ {
                    count += 1;
                } else {
                    push_compression(&mut output, last_char_, count);
                    last_char = Some(c);
                    count = 1;
                }
            }, 
            None => {
                last_char = Some(c);
                count = 1;
            }
        }
    }

    if let Some(last_char) = last_char {
        push_compression(&mut output, last_char, count);
    }

    if output.len() > input.len() {
        input.to_string()
    } else {
        output
    }
}

fn push_compression(output: &mut String, c: char, count: usize) {
    match count {
        0 => { }, 
        1 => output.push(c), 
        _ => {
            output.push(c);
            let digits = count.to_string();
            output.push_str(&digits);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_empty_when_compressing_then_remains_unchanged() {
        let sut = "";
        let res = super::compress(sut);
        assert_eq!(res, "");
    }

    #[test]
    fn given_single_char_when_compressing_then_remains_unchanged() {
        let sut = "a";
        let res = super::compress(sut);
        assert_eq!(res, "a");
    }

    #[test]
    fn given_single_char_multiple_times_when_compressing_then_is_compressed() {
        let sut = "aaaaa";
        let res = super::compress(sut);
        assert_eq!(res, "a5");
    }

    #[test]
    fn given_long_str_when_compressing_then_is_compressed() {
        let sut = "aaabbcxyyzzz";
        let res = super::compress(sut);
        assert_eq!(res, "a3b2cxy2z3");
    }

    #[test]
    fn given_non_compressable_when_compressing_then_is_compressed_without_saving_space() {
        let sut = "aabbcxyyzz";
        let res = super::compress(sut);
        assert_eq!(res, "a2b2cxy2z2");
    }
}
