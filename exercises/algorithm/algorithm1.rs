use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: Clone + Ord,
    {
        let mut merged_list = LinkedList::new();
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        // 合并两个链表直到其中一个遍历完
        while let (Some(a), Some(b)) = (current_a, current_b) {
            let a_node = unsafe { a.as_ref() };
            let b_node = unsafe { b.as_ref() };

            if a_node.val <= b_node.val {
                merged_list.add(a_node.val.clone());
                current_a = a_node.next;
            } else {
                merged_list.add(b_node.val.clone());
                current_b = b_node.next;
            }
        }

        // 处理剩余的链表A节点
        while let Some(a) = current_a {
            let a_node = unsafe { a.as_ref() };
            merged_list.add(a_node.val.clone());
            current_a = a_node.next;
        }

        // 处理剩余的链表B节点
        while let Some(b) = current_b {
            let b_node = unsafe { b.as_ref() };
            merged_list.add(b_node.val.clone());
            current_b = b_node.next;
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        assert_eq!(3, list.length);
        assert_eq!(1, *list.get(0).unwrap());
        assert_eq!(2, *list.get(1).unwrap());
        assert_eq!(3, *list.get(2).unwrap());
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        assert_eq!(3, list_str.length);
        assert_eq!("A", *list_str.get(0).unwrap());
        assert_eq!("B", *list_str.get(1).unwrap());
        assert_eq!("C", *list_str.get(2).unwrap());
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        
        let mut list_c = LinkedList::merge(list_a, list_b);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        
        let mut list_c = LinkedList::merge(list_a, list_b);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_empty_list() {
        let list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::new();
        list_b.add(1);
        list_b.add(2);
        
        let mut merged = LinkedList::merge(list_a, list_b);
        assert_eq!(2, merged.length);
        assert_eq!(1, *merged.get(0).unwrap());
        assert_eq!(2, *merged.get(1).unwrap());
    }
}