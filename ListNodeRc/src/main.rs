#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<NodeRc<T>>>;

struct NodeRc<T> {
    data: T,
    next: RefCell<Link<T>>, //Cell
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

struct ListNodeRc<T> {
    head: Link<T>,
}

struct IntoIterRc<T>(ListNodeRc<T>);

impl<T> Iterator for IntoIterRc<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let head = self.0.head.take()?;
        match Rc::try_unwrap(head) {
            Ok(node) => {
                self.0.head = node.next.into_inner();
                Some(node.data)
            }
            Err(_) => panic!("Multiple references to node in IntoIterRc"),
        }
    }
}

struct IterRc<T> {
    next: Option<Rc<NodeRc<T>>>,
}

impl<T> Iterator for IterRc<T> {
    type Item = Rc<NodeRc<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            let next = node.next.borrow().clone();
            self.next = next;
            node
        })
    }
}

// Iterator for IterMutRc реализовать нельзя, т.к. список построен на Rc, а Rc подразумевает совместное владение (&mut должен быть только 1)
struct IterMutRc<T> {
    next: Link<T>,
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
        if self.has_cycle() {
            panic!("Cycle detected in linked list!");
        }
    }

    fn get_node_at(&self, position: usize) -> Option<Rc<NodeRc<T>>> {
        self.iter_nodes().nth(position)
    }

    fn into_iter(self) -> IntoIterRc<T> {
        IntoIterRc(self)
    }

    fn iter(&self) -> IterRc<T> {
        IterRc {
            next: self.head.as_ref().map(Rc::clone),
        }
    }

    fn iter_mut(&mut self) -> IterMutRc<T> {
        IterMutRc {
            next: self.head.as_ref().map(Rc::clone),
        }
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

    fn append_at(&mut self, position: usize, data: T) {
        self.check_invariants();

        if position == 0 {
            self.push_head(data);
            return;
        }

        let prev_node = self.get_node_at(position - 1).expect("append_at failed");

        let new_node = Rc::new(NodeRc {
            data,
            next: RefCell::new(prev_node.next_node()),
        });

        *prev_node.next.borrow_mut() = Some(new_node);

        self.check_invariants();
    }

    fn remove_at(&mut self, position: usize) {
        self.check_invariants();

        if self.is_empty() {
            return;
        }

        let prev_node = self.get_node_at(position - 1).expect("remove_at failed");

        let node_to_remove = prev_node.next_node();

        if let Some(node) = node_to_remove {
            let next_node = node.next_node();
            prev_node.set_next(next_node);
        }

        self.check_invariants();
    }

    fn make_cycle_at(&mut self, position: usize) {
        self.check_invariants();
        if self.is_empty() {
            return;
        }

        let target_node = self
            .get_node_at(position - 1)
            .expect("make_cycle_at failed");

        let last_node = self.iter_nodes().last().expect("make_cycle_at failed");

        last_node.set_next(Some(Rc::clone(&target_node)));
    }

    fn has_cycle(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut slow_iter = self.iter_nodes();
        let mut fast_iter = self.iter_nodes();

        let mut slow = slow_iter.next();
        let mut fast = fast_iter.next();

        while slow.is_some() && fast.is_some() {
            slow = slow_iter.next();

            fast_iter.next();
            fast = fast_iter.next();

            if let (Some(s), Some(f)) = (&slow, &fast) {
                if Rc::ptr_eq(s, f) {
                    return true;
                }
            }
        }

        false
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

impl<T> Drop for ListNodeRc<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            if let Ok(node_inner) = Rc::try_unwrap(node) {
                current = node_inner.next.take();
            } else {
                break;
            }
        }
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
    list.extend(0..10_000);
}

impl<T> Extend<T> for ListNodeRc<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_head(item);
        }
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

#[test]
fn test_append_at() {
    let mut list = ListNodeRc::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);

    list.append_at(1, 9);
    let collected: Vec<_> = list.iter_nodes().map(|node| node.data).collect();
    assert_eq!(collected, vec![3, 9, 2, 1]);
}

#[test]
fn test_remove_at() {
    let mut list = ListNodeRc::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    list.push_head(4);

    list.remove_at(3);
    let collected: Vec<_> = list.iter_nodes().map(|node| node.data).collect();
    assert_eq!(collected, vec![4, 3, 2]);
}

#[test]
fn test_make_cycle_at() {
    let mut list = ListNodeRc::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    list.push_head(4);

    list.make_cycle_at(1);
    assert!(list.has_cycle());
}

// struct Ref<'a, T> {
//     data: *mut T,
//     _marker: PhantomData<&'a T>,
// }

// extern "C" {
//     fn make_ref(x: *mut NodeRc<u32>) -> *mut u32;
// }

// fn make_safe_ref(x: &'_ NodeRc<u32>) -> Ref<'_, u32> {
//     let data = unsafe {
//         make_ref((x as *const NodeRc<u32>).cast_mut())
//     };
//     Ref { data, _marker: PhantomData }
// }
