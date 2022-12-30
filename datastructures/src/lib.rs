pub mod linked_list {
    pub struct LinkedListNode<T> {
        pub value: T,
        // next: &LinkedListNode<T>
        pub next: Option<Box<LinkedListNode<T>>>
    }

    pub struct LinkedList<T> {
        pub head: Option<Box<LinkedListNode<T>>>
        // pub len: usize // Impl with memoization
    }

    impl <T> LinkedList<T> {
        pub fn lookup(&self, index: usize) -> Option<&T> {
            if let Some(node) = &self.head {
                let mut current_node = node;
                let mut current_index: usize = 0;

                while let Some(next) = &node.next {
                    if current_index < index {
                        current_node = &next;
                        current_index += 1;
                    }
                } 

                return Some(&current_node.value);
            }

            None
        }

        // Can't get this to work, Vec.append() uses unsafe Rust, so that's not great inspiration
        
        // pub fn append(&mut self, value: T) {
        //     if let Some(node) = &mut self.head {

        //         let current_node = &mut node;
    
        //         while let Some(mut next) = current_node.next {
        //             **current_node = next;
        //         }
    
        //         current_node.next = Some(Box::new(LinkedListNode {
        //             value,
        //             next: None
        //         }))
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list::*;
    
    #[test]
    fn test_lookup() {
        let mut test_list: LinkedList<usize> = LinkedList{head: Some(Box::new(LinkedListNode {value: 10, next: None}))};
        let result: Option<&usize> = Some(&10);

        assert_eq!(test_list.lookup(0), result);
    }

    #[test]
    fn test_lookup2() { // Caught in a loop?
        let mut test_list: LinkedList<usize> = LinkedList{head: Some(Box::new(LinkedListNode {value: 10, next: Some(Box::new(LinkedListNode {value: 1, next: None}))}))};
        let result: Option<&usize> = Some(&1);

        assert_eq!(test_list.lookup(1), result);
    }
}