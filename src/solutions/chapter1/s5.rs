use std::cmp::{min, max};

fn has_at_most_one_edit_been_made(orig: &str, modified: &str) -> bool {
    if orig == modified {
        true
    } else if has_char_been_replaced(orig, modified) {
        true
    } else if has_char_been_removed_or_inserted(orig, modified) {
        true
    } else {
        false
    }
}


// B(s) where s = min(len(b), len(a))
fn has_char_been_replaced(orig: &str, modified: &str) -> bool {
    match orig.len() {
        before_len if before_len != modified.len() => false, 
        0 => false,
        1 => orig != modified, 
        _ => {
            let mut found_replaced_char = false;

            let (mut orig_iter, mut modified_iter) = (orig.chars(), modified.chars());

            loop {
                let (orig_next, modified_next) = (orig_iter.next(), modified_iter.next());

                if orig_next.is_none() {
                    return found_replaced_char; // end of string
                }

                if orig_next != modified_next {
                    if found_replaced_char {
                        return false; // subsequent replaced char
                    }

                    found_replaced_char = true; // first replaced char
                }
            }
        }
    }
}


fn has_char_been_removed_or_inserted(orig: &str, modified: &str) -> bool {
    let (short, long) = 
        if orig.len() + 1 == modified.len() {
            (orig, modified)
        } else if modified.len() + 1 == orig.len() {
            (modified, orig)
        } else {
            return false;
        };

        let (mut short_iter, mut long_iter) = (short.chars(), long.chars());        
        let (mut short_next, mut long_next) = (short_iter.next(), long_iter.next());

        let mut found_char = false;

        loop {
            if short_next.is_none() { 
                if long_next.is_some() {
                    found_char = true;
                }

                break;
             }

            if short_next == long_next {
                short_next = short_iter.next();
            } else {
                if found_char { return false; }
                found_char = true;
            }

            long_next = long_iter.next();
        }

        found_char
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_empty_and_empty_when_checking_then_is_one_edit_away() {
        let orig = "";
        let modified = "";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_hello_and_hello_when_checking_then_is_one_edit_away() {
        let orig = "hello";
        let modified = "hello";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_empty_and_a_when_checking_then_is_one_edit_away() {
        let orig = "";
        let modified = "a";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_a_and_empty_when_checking_then_is_one_edit_away() {
        let orig = "a";
        let modified = "";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_a_and_b_when_checking_then_is_one_edit_away() {
        let orig = "a";
        let modified = "b";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_hello_and_helo_when_checking_then_is_one_edit_away() {
        let orig = "hello";
        let modified = "helo";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_hello_and_helllo_when_checking_then_is_one_edit_away() {
        let orig = "hello";
        let modified = "helllo";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_hello_and_hella_when_checking_then_is_one_edit_away() {
        let orig = "hello";
        let modified = "hella";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(res);
    }

    #[test]
    fn given_hello_and_world_when_checking_then_is_not_one_edit_away() {
        let orig = "hello";
        let modified = "world";
        let res = super::has_at_most_one_edit_been_made(orig, modified);
        assert!(!res);
    }
}