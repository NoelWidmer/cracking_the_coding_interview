const SPACE_CHAR: u8 = ' ' as u8;
const PADDING_CHAR: u8 = '\0' as u8;

fn encode_space(url: &mut [u8]) {
    let url_len = url.len();

    if url_len == 0 {
        return;
    }

    let last_index = url_len - 1;
    let mut cursor = last_index;
    let mut target = last_index;

    loop {
        let current_char = url[cursor];

        if current_char != PADDING_CHAR {
            let subtract_number;
            
            if current_char == SPACE_CHAR {
                url[target] = '0' as u8;
                url[target - 1] = '2' as u8;
                url[target - 2] = '%' as u8;
                subtract_number = 3;
            } else {
                url[target] = current_char;
                subtract_number = 1;
            }

            if target >= subtract_number {
                target -= subtract_number;
            } else {
                return; 
            }
        }

        if cursor == 0 {
            break;
        } else {
            cursor -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_empty_when_encoding_then_remains_unchanged() {
        let mut sut = given("");
        when_encoding(&mut sut);
        then_equals(sut, "");
    }
    #[test]
    fn given_spaces_when_encoding_then_spaces_are_encoded() {
        let mut sut = given("   \0\0\0\0\0\0");
        when_encoding(&mut sut);
        then_equals(sut, "%20%20%20");
    }

    #[test]
    fn given_single_word_when_encoding_then_remains_unchanged() {
        let mut sut = given("hello");
        when_encoding(&mut sut);
        then_equals(sut, "hello");
    }

    #[test]
    fn given_hello_world_when_encoding_then_space_is_encoded() {
        let mut sut = given("hello world\0\0");
        when_encoding(&mut sut);
        then_equals(sut, "hello%20world");
    }

    #[test]
    fn given_multiple_words_when_encoding_then_spaces_are_encoded() {
        let mut sut = given("hello world; goodbye hell\0\0\0\0\0\0");
        when_encoding(&mut sut);
        then_equals(sut, "hello%20world;%20goodbye%20hell");
    }

    fn given(sut: &str) -> Vec<u8> {
        sut.as_bytes().to_vec()
    }

    fn when_encoding(sut: &mut [u8]) {
        super::encode_space(sut);
    }

    fn then_equals(sut: Vec<u8>, expected: &str) {
        assert_eq!(sut, expected.as_bytes().to_vec());
    }
}