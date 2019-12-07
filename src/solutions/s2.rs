use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn is_permutation(a: &str, b: &str) -> bool {
    let a_len = a.len();
    let b_len = b.len();

    if a_len != b_len  {
        false
    } else if a_len == 0 {
        true
    } else if a_len == 1 {
        a == b
    } else {
        let mut lookup = HashMap::new();

        for c in a.chars() {
            lookup
                .entry(c)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        for c in b.chars() {
            match lookup.entry(c) {
                Entry::Occupied(mut e) => {
                    let count = e.get_mut();

                    if count > &mut 1 {
                        *count -= 1;
                    } else {
                        e.remove_entry();
                    }
                }, 
                Entry::Vacant(_) => return false
            }
        }

        lookup.len() == 0 // must be empty now
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_two_strings_with_different_lengths_when_checking_then_is_no_permutation() {
        let sut = given("hello", "hello ");
        let res = when_checking_for_permutation(sut);
        then_is_no_permutation(res);
    }

    #[test]
    fn given_two_empty_strings_when_checking_then_is_permutation() {
        let sut = given("", "");
        let res = when_checking_for_permutation(sut);
        then_is_permutation(res);
    }

    #[test]
    fn given_two_single_ascii_char_strings_when_checking_then_is_permutation() {
        let sut = given("a", "a");
        let res = when_checking_for_permutation(sut);
        then_is_permutation(res);
    }

    #[test]
    fn given_two_single_ascii_char_strings_when_checking_then_is_no_permutation() {
        let sut = given("a", "b");
        let res = when_checking_for_permutation(sut);
        then_is_no_permutation(res);
    }

    #[test]
    fn given_two_single_unicode_char_strings_when_checking_then_is_permutation() {
        let sut = given("ö", "ö");
        let res = when_checking_for_permutation(sut);
        then_is_permutation(res);
    }

    #[test]
    fn given_two_single_unicode_char_strings_when_checking_then_is_no_permutation() {
        let sut = given("ö", "ä");
        let res = when_checking_for_permutation(sut);
        then_is_no_permutation(res);
    }

    #[test]
    fn given_two_strings_when_checking_then_is_permutation() {
        let sut = given("hello world", "werhd lolol");
        let res = when_checking_for_permutation(sut);
        then_is_permutation(res);
    }

    #[test]
    fn given_two_strings_with_unique_chars_when_checking_then_is_no_permutation() {
        let sut = given("abc . def", "abc x def");
        let res = when_checking_for_permutation(sut);
        then_is_no_permutation(res);
    }

    #[test]
    fn given_two_strings_with_similar_chars_when_checking_then_is_no_permutation() {
        let sut = given("abcdef ", "abcdef b");
        let res = when_checking_for_permutation(sut);
        then_is_no_permutation(res);
    }

    fn given<'a, 'b>(a: &'a str, b: &'b str) -> (&'a str, &'b str) {
        (a, b)
    }

    fn when_checking_for_permutation(strings: (&str, &str)) -> bool {
        super::is_permutation(strings.0, strings.1)
    }

    fn then_is_permutation (result: bool) {
        assert!(result)
    }

    fn then_is_no_permutation (result: bool) {
        assert!(!result)
    }
}