type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T : Copy> {
    value: T,
    next: Link<T>,
}

fn new_link<T: Copy>(n: T) -> Link<T> {
    Some(Box::new(Node::new(n)))
}

impl<T: Copy> Node<T> {
    fn new(n: T) -> Self {
        Node {
            value: n,
            next: None,
        }
    }

    fn add(&mut self, n: T) {
        match self.next {
            Some(ref mut boxed) => boxed.add(n),
            None => {
                self.next = new_link(n);
            }
        }
    }

    fn get(&self, i: u32) -> T {
        if i == 0 {
            self.value
        } else {
            match self.next {
                Some(ref boxed) => boxed.get(i - 1),
                None => panic!("Out of bound")
            }
        }
    }
}

pub struct LinkedList<T: Copy> {
    head: Link<T>
}

impl<T: Copy> LinkedList<T> {
    pub fn empty() -> Self {
        LinkedList {
            head: None
        }
    }
    fn is_empty(&self) -> bool {
        match self.head {
            None => true,
            _ => false,
        }
    }
    fn add(&mut self, n: T) {
        match self.head {
            Some(ref mut h) => h.add(n),
            None => self.head = new_link(n)
        }
    }

    fn get(&self, i: u32) -> T {
        match self.head {
            Some(ref h) => h.get(i),
            None => panic!("out of bound")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn empty() {
        let l = LinkedList::<u32>::empty();
        assert_that!(l.is_empty()).is_true();
    }

    #[test]
    fn add() {
        let mut l = LinkedList::empty();
        l.add(2);
        assert_that!(l.is_empty()).is_false();
    }

    #[test]
    fn get() {
        let mut l = LinkedList::empty();
        l.add(2);
        assert_that!(l.get(0)).is_equal_to(2);
    }

    #[test]
    fn get_second() {
        let mut l = LinkedList::empty();
        l.add(2);
        l.add(4);
        assert_that!(l.get(1)).is_equal_to(4);
    }

    #[test]
    #[should_panic]
    fn get_empty() {
        let l = LinkedList::<i32>::empty();
        l.get(0);
    }
}
