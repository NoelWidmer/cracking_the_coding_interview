use std::collections::HashMap;

fn is_permutation_of_palindrome(input: &str) -> bool {
    match input.len() {
        0 | 1 => true, 
        _ => {
            let mut lookup = HashMap::new();

            for c in input.chars() {
                if c != ' ' { // we ignore spaces
                    lookup
                        .entry(c)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }

            let mut found_middle = false;

            for e in lookup {
                if e.1 % 2 != 0 {
                    if found_middle {
                        return false;
                    }

                    found_middle = true;
                }
            }

            true
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    fn given_empty_when_checking_then_is_permutation() {
        let sut = given("");
        let res = when_checking(sut);
        then_is_permutation(res);
    }

    #[cfg(test)]
    fn given_whitespace_when_checking_then_is_permutation() {
        let sut = given("   ");
        let res = when_checking(sut);
        then_is_permutation(res);
    }

    #[cfg(test)]
    fn given_permutation_of_palindrome_when_checking_then_is_permutation() {
        let sut = given("guuss");
        let res = when_checking(sut);
        then_is_permutation(res);
    }

    #[cfg(test)]
    fn given_palindrome_when_checking_then_is_permutation() {
        let sut = given("sugus");
        let res = when_checking(sut);
        then_is_permutation(res);
    }

    #[cfg(test)]
    fn given_permutation_of_palindrome_with_space_when_checking_then_is_permutation() {
        let sut = given("ccaat ");
        let res = when_checking(sut);
        then_is_permutation(res);
    }

    #[cfg(test)]
    fn given_five_different_chars_when_checking_then_is_not_a_permutation() {
        let sut = given("abxbdad");
        let res = when_checking(sut);
        then_is_not_a_permutation(res);
    }

    fn given(sut: &str) -> &str {
        sut
    }

    fn when_checking(sut: &str) -> bool {
        super::is_permutation_of_palindrome(sut)
    }

    fn then_is_permutation(res: bool) {
        assert!(res);
    }

    fn then_is_not_a_permutation(res: bool) {
        assert!(!res);
    }
}