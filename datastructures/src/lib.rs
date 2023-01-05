pub mod linked_list {
    use std::cell::RefCell;

    pub struct ListNode<T> {
        pub value: T,
        pub len: Option<RefCell<usize>>,
        pub next: Option<Box<ListNode<T>>>
    }

    impl <T: Copy> ListNode<T> {
        pub fn new(value: T) -> ListNode<T> {
            ListNode {
                value,
                len: None,
                next: None
            }
        }

        pub fn append(&mut self, value: T) {
            if let Some(ref mut node) = self.next {
                node.append(value);
            } else {
                self.next = Some(Box::new(ListNode::new(value)));
            }
        }

        pub fn nth(&self, index: usize) -> Option<T> {
            if index == 0 {
                let output = self.value;
                return Some(output);
            }

            let mut current_index = 0;
            let mut current_node = self;

            while let Some(node) = &current_node.next {
                if current_index < index {
                    current_node = &node;
                    current_index += 1;
                }

                if current_index == index {
                    let output = current_node.value;
                    return Some(output);
                }
            }
            None
        }

        pub fn len(&self) -> usize { // Doesn't work yet
            if let Some(length) = self.len {
                return length;
            }
            else {
                let mut counter: usize = 1;
                let mut current_node = self;

                while let Some(node) = &current_node.next {
                    current_node = &node;
                    counter += 1;
                }
                self.len = Some(RefCell::new(counter));
                counter    
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list::*;

    #[test]
    fn test_create() {
        let new_list: ListNode<usize> = ListNode::new(5);

        assert_eq!(new_list.value, 5 as usize);
    }

    #[test]
    fn test_append() {
        let mut new_list: ListNode<usize> = ListNode::new(5);
        new_list.append(6);

        let mut value: usize = 0;

        if let Some(node) = new_list.next {
            value = node.value;
        }

        assert_eq!(value, 6);
    }

    #[test]
    fn test_nth() {
        let mut new_list: ListNode<usize> = ListNode::new(5);
        new_list.append(6);
        new_list.append(7);
        new_list.append(8);

        assert_eq!(Some(8), new_list.nth(3));
    }

    #[test]
    fn test_len() {
        let mut new_list: ListNode<usize> = ListNode::new(5);
        new_list.append(6);
        new_list.append(7);
        new_list.append(8);

        assert_eq!(4, new_list.len());
    }
}