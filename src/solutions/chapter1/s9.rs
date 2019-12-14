fn is_rotation_of(orig: &str, rotation: &str) -> bool {
    if orig.len() == rotation.len() {
        if orig.len() == 0 {
            true
        } else {
            let mut orig_twice = String::with_capacity(orig.len() * 2);
            orig_twice.push_str(orig);
            orig_twice.push_str(orig);
            orig_twice.contains(rotation)
        }
    } else {
        false
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn given_empty_when_checking_if_empty_is_a_rotation_then_is_true() {
        let val = given("", "");
        let res = when_checking(val);
        then_is_rotation(res);
    }

    #[test]
    fn given_hello_when_checking_if_hello_is_a_rotation_then_is_true() {
        let val = given("hello", "hello");
        let res = when_checking(val);
        then_is_rotation(res);
    }

    #[test]
    fn given_hello_when_checking_if_lohel_is_a_rotation_then_is_true() {
        let val = given("hello", "lohel");
        let res = when_checking(val);
        then_is_rotation(res);
    }

    #[test]
    fn given_hello_when_checking_if_llohel_is_a_rotation_then_is_false() {
        let val = given("hello", "llohel");
        let res = when_checking(val);
        then_is_not_a_rotation(res);
    }

    fn given<'a, 'b>(orig: &'a str, rotation: &'b str) -> (&'a str, &'b str) {
        (orig, rotation)
    }

    fn when_checking(val: (&str, &str)) -> bool {
        super::is_rotation_of(val.0, val.1)
    }

    fn then_is_rotation(res: bool) {
        assert!(res);
    }

    fn then_is_not_a_rotation(res: bool) {
        assert!(!res);
    }
}