fn are_all_unique<T: Ord + Clone>(items: &mut [T]) -> bool {
    items.sort_unstable();

    let mut prev: Option<&T> = None;

    for item in items {
        if let Some(prev) = prev {
            if prev == item {
                return false;
            }
        }
        
        prev = Some(item);
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_empty_when_checking_for_uniqueness_then_are_unique() {
        let sut: Vec<i32> = Vec::new();
        let res = when_checking_uniqueness(sut);
        then_are_unique(res);
    }

    #[test]
    fn given_single_item_when_checking_for_uniqueness_then_are_unique() {
        let sut = vec![ 177 ];
        let res = when_checking_uniqueness(sut);
        then_are_unique(res);
    }

    #[test]
    fn given_two_unique_items_when_checking_for_uniqueness_then_are_unique() {
        let sut = vec![ 177, 666 ];
        let res = when_checking_uniqueness(sut);
        then_are_unique(res);
    }

    #[test]
    fn given_two_non_unique_items_when_checking_for_uniqueness_then_are_not_unique() {
        let sut = vec![ 117, 117 ];
        let res = when_checking_uniqueness(sut);
        then_are_not_unique(res);
    }

    #[test]
    fn given_multiple_unqiue_items_when_checking_for_uniqueness_then_are_unique() {
        let sut = vec![ 177, 666, 22, 99, 11, 1, 33, 0 ];
        let res = when_checking_uniqueness(sut);
        then_are_unique(res);
    }

    #[test]
    fn given_multiple_non_unqiue_items_when_checking_for_uniqueness_then_are_not_unique() {
        let sut = vec![ 177, 666, 22, 99, 11, 117, 33, 22, 0 ];
        let res = when_checking_uniqueness(sut);
        then_are_not_unique(res);
    }

    fn when_checking_uniqueness(mut sut: Vec<i32>) -> bool {
        super::are_all_unique(&mut sut)
    }

    fn then_are_unique(res: bool) {
        assert!(res);
    }

    fn then_are_not_unique(res: bool) {
        assert!(!res);
    }
}
