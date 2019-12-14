use std::collections::LinkedList;
use std::collections::HashSet;
use std::hash::Hash;

fn remove_duplicates<T: Eq + Hash + Clone>(list: &mut LinkedList<T>) {
     if list.len() < 2 { return; }

     let mut lookup = HashSet::new();

     for _ in 0..list.len() {
         let val = list.pop_back().unwrap();
         let val_ = val.clone();

         if lookup.insert(val) {
             list.push_front(val_);
         }
     }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn given_empty_list_when_removing_duplicates_then_list_is_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();
        super::remove_duplicates(&mut list);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn given_list_with_one_item_when_removing_duplicates_then_list_remains_unchanged() {
        let mut list = LinkedList::new();
        list.push_front(5);
        let expected = list.clone();
        super::remove_duplicates(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn given_list_with_unique_items_when_removing_duplicates_then_list_remains_unchanged() {
        let mut list = LinkedList::new();
        list.push_front(5);
        list.push_front(4);
        list.push_front(3);
        list.push_front(2);
        let expected = list.clone();
        super::remove_duplicates(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn given_list_with_duplicate_items_when_removing_duplicates_then_list_contains_only_unique_items() {
        let mut list = LinkedList::new();
        list.push_front(5);
        list.push_front(4);
        list.push_front(5);
        list.push_front(2);
        let mut expected = LinkedList::new();
        expected.push_front(5);
        expected.push_front(4);
        expected.push_front(2);
        super::remove_duplicates(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn given_list_with_all_same_items_when_removing_duplicates_then_list_contains_only_one_item() {
        let mut list = LinkedList::new();
        list.push_front(5);
        list.push_front(5);
        list.push_front(5);
        list.push_front(5);
        let mut expected = LinkedList::new();
        expected.push_front(5);
        super::remove_duplicates(&mut list);
        assert_eq!(list, expected);
    }
}