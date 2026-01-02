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
    // fn head(&self) -> Link<T> {
    //     self.head.as_ref().cloned()
    // }

    // fn head_mut(&mut self) -> &mut Link<T> {
    //     &mut self.head
    // }

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

    fn get_node_at(&self, position: usize) -> Option<T>
    where T: Clone
    {
        self.iter_nodes().nth(position).map(|node| node.data.clone())
    }

    // fn into_iter(self) -> IntoIter<T> {
    //     IntoIter(self)
    // }

    // fn iter(&self) -> Iter<'_, T> {
    //     Iter {
    //         next: self.head.as_ref().map(Rc::clone),
    //         _marker: PhantomData,
    //     }
    // }

    // fn iter_mut(&mut self) -> IterMut<'_, T> {
    //     IterMut {
    //         next: self.head.as_ref().map(Rc::clone),
    //         _marker: PhantomData,
    //     }
    // }

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

//     fn push_head(&mut self, data: T) {
//         self.check_invariants();

//         let new_head = Rc::new(NodeRc {
//             data,
//             next: RefCell::new(self.head.take()),
//         });

//         self.head = Some(new_head);

//         self.check_invariants();
//     }

    fn join(self, other: Self) -> Self {
        self.check_invariants();

        if other.is_empty() {
            return self;
        }

        if self.is_empty() {
            self.head = other.head.take();
            return;
        }

        if let Some(last_node) = self.iter_nodes().last() {
            last_node.set_next(other.head.take());
        }

        self.check_invariants();
    }

//     fn divide_at(&mut self, position: usize) -> Option<(Self, Self)> {
//         self.check_invariants();
//         if self.is_empty() {
//             return None;
//         }

//         if position == 0 {
//             return Some((Self::new(), std::mem::take(self)));
//         }

//         let prev_node = self.get_node_at(position - 1)?;

//         let head2 = prev_node.next_node();
//         prev_node.set_next(None);

//         let list1 = ListNodeRc {
//             head: self.head.take(),
//         };

//         let list2 = ListNodeRc { head: head2 };

//         self.check_invariants();
//         Some((list1, list2))
//     }

//     fn append_at(&mut self, position: usize, data: T) {
//         self.check_invariants();

//         if position == 0 {
//             self.push_head(data);
//             return;
//         }

//         let prev_node = self
//             .get_node_at(position - 1).ok_or(())?; //TODO ?

//         let new_node = Rc::new(NodeRc {
//             data,
//             next: RefCell::new(prev_node.next_node()),
//         });

//         *prev_node.next.borrow_mut() = Some(new_node);

//         self.check_invariants();
//     }

//     fn remove_at(&mut self, position: usize) {
//         self.check_invariants();

//         if self.is_empty() {
//             return;
//         }

//         let prev_node = self.get_node_at(position - 1).unwrap(); //TODO ?

//         let node_to_remove = prev_node.next_node();

//         if let Some(node) = node_to_remove {
//             let next_node = node.next_node();
//             prev_node.set_next(next_node);
//         }

//         self.check_invariants();
//     }

//     fn make_cycle_at(&mut self, position: usize) {
//         self.check_invariants();
//         if self.is_empty() {
//             return;
//         }

//         let target_node = self.get_node_at(position - 1).unwrap(); //TODO ?

//         let last_node = self.iter_nodes().last().unwrap(); //TODO ?

//         last_node.set_next(Some(Rc::clone(&target_node)));

//         self.check_invariants();
//     }

//     fn has_cycle(&self) -> bool {
//         self.check_invariants();

//         if self.is_empty() {
//             return false;
//         }

//         let mut slow_iter = self.iter_nodes();
//         let mut fast_iter = self.iter_nodes();

//         let mut slow = slow_iter.next();
//         let mut fast = fast_iter.next();

//         while slow.is_some() && fast.is_some() {
//             slow = slow_iter.next();

//             fast_iter.next();
//             fast = fast_iter.next();

//             if let (Some(s), Some(f)) = (&slow, &fast) {
//                 if Rc::ptr_eq(s, f) {
//                     return true;
//                 }
//             }
//         }

//         self.check_invariants();
//         false
//     }

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

// impl<T: Clone> Iterator for IntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0.is_empty() {
//             return None;
//         }

//         let current_head = self.0.head.take().unwrap();

//         let current_ref = current_head.borrow();
//         let data = current_ref.data.clone();
//         let next_node = current_ref.next.as_ref().map(Rc::clone);

//         self.0.head = next_node;

//         if self.0.is_empty() {
//             self.0.tail = None;
//         }

//         Some(data)
//     }
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             // let node_ref = node.borrow();
//             // self.next = node_ref.next.as_ref().map(Rc::clone);

//             // unsafe { &*(&node_ref.data as *const T) } /////////////
//             self.next = node.next.as_ref().map(Rc::clone);
//             &node.data
//         })
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             let mut node_ref = node.borrow_mut();
//             self.next = node_ref.next.as_ref().map(Rc::clone);

//             unsafe { &mut *(&mut node_ref.data as *mut T) }
//         })
//     }
// }

// impl<T> Drop for ListNodeRc<T> {
//     fn drop(&mut self) {
//         let mut current = self.head.take();
//         while let Some(node) = current {
//             if let Ok(mut node_inner) = Rc::try_unwrap(node) {
//                 current = node_inner.get_mut().next.take();
//             } else {
//                 break;
//             }
//         }
//         self.tail.take();
//     }
// }

fn main() {
    // let mut list = ListNodeRc::new();

    //     list.push_back(1);
    //     list.push_back(2);
    //     list.push_back(3);
    //     println!("after push_back: {:?}\n", list);

    //     list.pop_back();
    //     list.pop_back();
    //     println!("after pop_back: {:?}\n", list);
    //     // //////////////////////////////////////////////////////////////////
    //     let mut list1 = ListNodeRc::new();
    //     list1.push_back(1);

    //     let mut list2 = ListNodeRc::new();
    //     list2.push_back(2);
    //     list2.push_back(3);

    //     list1.join(&mut list2);

    //     println!("After append: {:?}\n", list1);
    //     println!("List 2 after append: {:?}\n", list2);
    //     //////////////////////////////////////////////////////////////////
    //     let (first, second) = list1.divide_at(1).unwrap();

    //     println!("After division\n");
    //     println!("First part: {:?}\n", first);
    //     println!("Second part: {:?}\n", second);
    //     //////////////////////////////////////////////////////////////////
    //     let mut list = ListNodeRc::new();

    //     list.push_back(1);
    //     list.push_back(2);
    //     list.push_back(3);
    //     list.append_at(1, 10);
    //     println!("After appending element at position: {:?}\n", list);
    //     //////////////////////////////////////////////////////////////////
    //     let mut list = ListNodeRc::new();

    //     list.push_back(1);
    //     list.push_back(2);
    //     list.push_back(3);
    //     list.remove_at(1);
    //     println!("After removing element at position: {:?}\n", list);
    //     //////////////////////////////////////////////////////////////////
    //     let mut list = ListNodeRc::new();
    //     list.push_back(1);
    //     list.push_back(2);
    //     list.push_back(3);
    //     println!("Iter\n");

    //     let mut iter = list.into_iter();
    //     assert_eq!(iter.next(), Some(1));
    //     assert_eq!(iter.next(), Some(2));
    //     assert_eq!(iter.next(), Some(3));
    //     assert_eq!(iter.next(), None);
}

// // проверка отсутствия переполнения стека
// fn create_and_drop_large_list() {
//     let mut list = ListNodeRc::new();
//     for i in 0..1_000_000 {
//         list.push_back(i);
//     }
// }

// #[test]
// fn test_large_list_drop() {
//     create_and_drop_large_list();
// }

// #[test]
// fn test_make_cycle_and_detect() {
//     let mut list = ListNodeRc::new();
//     list.push_back(1);
//     list.push_back(2);
//     list.push_back(3);
//     list.push_back(4);
//     list.push_back(5);

//     assert!(!list.has_cycle());

//     list.make_cycle_at(2);

//     assert!(list.has_cycle());
// }
