use std::cmp::Ordering;

pub fn merge_sort<T: Ord>(items: &[T]) -> Vec<&T> {
    match items.len() {
        0 => Vec::new(), 
        1 => vec![ &items[0] ],
        2 => {
            let first = &items[0];
            let second = &items[1];

            match first.cmp(second) {
                Ordering::Less | Ordering::Equal => vec![ first, second ],
                Ordering::Greater => vec![ second, first ],
            }
        }, 
        _ => {
            let middle = items.len() / 2;

            let left_sorted = merge_sort(&items[..middle]);
            let right_sorted = merge_sort(&items[middle..]);

            merge(&left_sorted, &right_sorted)
        }
    }
}

fn merge<'a, T: Ord>(left: &[&'a T], right: &[&'a T]) -> Vec<&'a T> {
    let mut result = Vec::with_capacity(left.len() + right.len());

    let mut next_left_index = 0;
    let mut next_right_index = 0;

    loop {
        if left.len() > next_left_index {
            let next_left = left[next_left_index];

            if right.len() > next_right_index {
                let next_right = right[next_right_index];
                
                match next_left.cmp(next_right) {
                    Ordering::Less => {
                        result.push(next_left);
                        next_left_index += 1;
                    }, 
                    Ordering::Equal => {
                        result.push(next_left);
                        result.push(next_right);
                        next_left_index += 1;
                        next_right_index += 1;
                    }, 
                    Ordering::Greater => {
                        result.push(next_right);
                        next_right_index += 1;
                    }
                }
            } else {
                result.push(next_left);
                next_left_index += 1;
            }
        } else {
            if right.len() > next_right_index {
                result.push(right[next_right_index]);
                next_right_index += 1;
            } else {
                break; // merge completed
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    // short input
    #[test]
    fn given_empty_vec_when_sorted_then_empty_vec_is_returned() {
        let sut: Vec<i32> = Vec::new();
        let result = super::merge_sort(&sut);
        let expected: Vec<&i32> = Vec::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn given_single_item_when_sorted_then_single_item_is_returned() {
        let sut = vec![ 177 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &177 ]);
    }

    #[test]
    fn given_two_different_items_when_sorted_then_order_is_asc() {
        let sut = vec![ 666, 117 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &117, &666 ]);
    }

    #[test]
    fn given_two_equal_items_when_sorted_then_order_is_asc() {
        let sut = vec![ 117, 666 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &117, &666 ]);
    }

    // long input
    
    #[test]
    fn given_large_vec_when_sorted_then_sorted_asc() {
        let sut = vec![ 9, 14, 6, 2, 15, 13, 5, 11, 4, 3, 12, 1, 10, 8, 0, 7 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15 ]);
    }
    
    #[test]
    fn given_large_vec_sorted_asc_when_sorted_then_remains_the_same() {
        let sut = vec![ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15 ]);
    }
    
    #[test]
    fn given_large_vec_sorted_desc_when_sorted_then_sorted_asc() {
        let sut = vec![ 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15 ]);
    }
}