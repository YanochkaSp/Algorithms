use std::{cell::RefCell, collections::HashSet, fmt::Debug, marker::PhantomData, rc::Rc};

type Link<T> = Option<Rc<NodeRc<T>>>;

#[derive(Debug, PartialEq)]
struct NodeRc<T> {
    data: T,
    next: RefCell<Link<T>>,
}

impl<T> NodeRc<T> {
    fn next_node(&self) -> Link<T> {
        let next_ref = self.next.borrow();
        next_ref.as_ref().map(Rc::clone)
    }

    fn set_next(&self, next: Link<T>) {
        *self.next.borrow_mut() = next;
    }
}

#[derive(Debug)]
struct ListNodeRc<T> {
    head: Link<T>,
}

struct IntoIter<T>(ListNodeRc<T>);

struct Iter<'a, T> {
    next: Link<T>,
    _marker: PhantomData<&'a T>,
}

struct IterMut<'a, T> {
    next: Link<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<T> Default for ListNodeRc<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> ListNodeRc<T> {
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn check_invariants(&self) {
        let mut visited = HashSet::new();
        let mut current = self.head.clone();

        while let Some(node_rc) = current {
            let node_ptr = Rc::as_ptr(&node_rc);
            if visited.contains(&node_ptr) {
                panic!("Cycle detected in linked list!");
            }
            visited.insert(node_ptr);

            let node = &node_rc;

            let next_borrow = node.next.borrow();
            current = next_borrow.clone();
        }
    }

    fn get_node_at(&self, position: usize) -> Option<Rc<NodeRc<T>>> {
        self.iter_nodes().nth(position)
    }

    fn new() -> Self {
        Self::default()
    }

    fn pop_head(&mut self) -> Option<T> {
        self.check_invariants();

        let old_head = self.head.take()?;

        self.head = old_head.next_node();

        self.check_invariants();

        match Rc::try_unwrap(old_head) {
            Ok(node) => Some(node.data),
            Err(_) => panic!("Cannot pop head"),
        }
    }

    fn push_head(&mut self, data: T) {
        self.check_invariants();

        let new_head = Rc::new(NodeRc {
            data,
            next: RefCell::new(self.head.take()),
        });

        self.head = Some(new_head);

        self.check_invariants();
    }

    fn join(self, mut other: Self) -> Self {
        self.check_invariants();
        other.check_invariants();

        if other.is_empty() {
            return self;
        }

        if self.is_empty() {
            return other;
        }

        if let Some(last_node) = self.iter_nodes().last() {
            last_node.set_next(other.head.take());
        }

        self.check_invariants();
        self
    }

    fn divide_at(&mut self, position: usize) -> Option<(Self, Self)> {
        self.check_invariants();
        if self.is_empty() {
            return None;
        }

        if position == 0 {
            return Some((Self::new(), std::mem::take(self)));
        }

        let prev_node = self.get_node_at(position - 1)?;

        let head2 = prev_node.next_node();
        prev_node.set_next(None);

        let list1 = ListNodeRc {
            head: self.head.take(),
        };

        let list2 = ListNodeRc { head: head2 };

        self.check_invariants();
        Some((list1, list2))
    }

    fn iter_nodes(&self) -> NodeIter<T> {
        NodeIter {
            next: self.head.as_ref().map(Rc::clone),
        }
    }
}

struct NodeIter<T> {
    next: Link<T>,
}

impl<T> Iterator for NodeIter<T> {
    type Item = Rc<NodeRc<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let next = {
                let node_ref = node.next.borrow();
                node_ref.as_ref().map(Rc::clone)
            };
            self.next = next;
            node
        })
    }
}

fn main() {
    let mut list = ListNodeRc::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    let collected1: Vec<_> = list.iter_nodes().map(|node| node.data).collect();
    assert_eq!(collected1, vec![3, 2, 1]);

    list.pop_head();
    list.pop_head();
    let collected2: Vec<_> = list.iter_nodes().map(|node| node.data).collect();
    assert_eq!(collected2, vec![1]);
}

// проверка отсутствия переполнения стека
fn create_and_drop_large_list() {
    let mut list = ListNodeRc::new();
    for i in 0..1_000_000 {
        list.push_head(i);
    }
}

#[test]
fn test_large_list_drop() {
    create_and_drop_large_list();
}

#[test]
fn test_join() {
    let mut a = ListNodeRc::new();
    a.push_head(1);
    a.push_head(2);

    let mut b = ListNodeRc::new();
    b.push_head(3);
    b.push_head(4);

    let joined = a.join(b);

    let values: Vec<_> = joined.iter_nodes().map(|node| node.data).collect();
    assert_eq!(values, vec![2, 1, 4, 3]);
}

#[test]
fn test_divide_at() {
    let mut list = ListNodeRc::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    list.push_head(4);
    list.push_head(5);
    let (a, b) = list.divide_at(3).expect("divide_at failed");
    let left: Vec<_> = a.iter_nodes().map(|node| node.data).collect();
    let right: Vec<_> = b.iter_nodes().map(|node| node.data).collect();

    assert_eq!(left, vec![5, 4, 3]);
    assert_eq!(right, vec![2, 1]);
}
