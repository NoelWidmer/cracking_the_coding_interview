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

            merge(left_sorted, right_sorted)
        }
    }
}

fn merge<'a, T: Ord>(left: Vec<&'a T>, right: Vec<&'a T>) -> Vec<&'a T> {
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
    #[test]
    fn no_item_gets_sorted_correctly() {
        let sut: Vec<i32> = Vec::new();
        let result = super::merge_sort(&sut);
        let expected: Vec<&i32> = Vec::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn single_item_gets_sorted_correctly() {
        let sut = vec![ 177 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &177 ]);
    }

    #[test]
    fn two_items_get_sorted_correctly() {
        let sut = vec![ 666, 117 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &117, &666 ]);
    }

    #[test]
    fn two_sorted_items_get_sorted_correctly() {
        let sut = vec![ 177, 666 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &117, &666 ]);
    }

    #[test]
    fn multiple_items_get_sorted_correctly() {
        let sut = vec![ 177, 666, 22, 22, 666, 1, 33, 666 ];
        let result = super::merge_sort(&sut);
        assert_eq!(result, vec![ &1, &22, &22, &33, &177, &666, &666, &666 ]);
    }
}