use crate::algorithms::merge_sort;

fn are_all_unique<T: Ord>(items: Vec<T>) -> bool {
    let sorted_items = merge_sort(&items[..]);

    let mut prev: Option<&T> = None;

    for item in sorted_items {
        if prev == Some(item) {
            return false;
        }

        prev = Some(item);
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_item_is_unique() {
        let sut: Vec<usize> = Vec::new();
        let result = super::are_all_unique(sut);
        assert!(result);
    }

    #[test]
    fn single_item_is_unique() {
        let sut = vec![ 177 ];
        let result = super::are_all_unique(sut);
        assert!(result);
    }

    #[test]
    fn two_items_are_unique() {
        let sut = vec![ 177, 666 ];
        let result = super::are_all_unique(sut);
        assert!(result);
    }

    #[test]
    fn two_items_are_not_unique() {
        let sut = vec![ 177, 117 ];
        let result = super::are_all_unique(sut);
        assert!(result);
    }

    #[test]
    fn multiple_items_are_unique() {
        let sut = vec![ 177, 666, 22, 99, 11, 1, 33, 0 ];
        let result = super::are_all_unique(sut);
        assert!(result);
    }

    #[test]
    fn multiple_items_are_not_unique() {
        let sut = vec![ 177, 666, 22, 99, 11, 117, 33, 0 ];
        let result = super::are_all_unique(sut);
        assert!(result);
    }
}
