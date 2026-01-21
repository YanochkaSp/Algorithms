#![allow(unused)]

type Link<T> = Option<Box<NodeBox<T>>>;

struct NodeBox<T> {
    data: T,
    next: Link<T>,
}

impl<T> NodeBox<T> {
    fn set_next(&mut self, next: Link<T>) {
        self.next = next;
    }
}

struct IntoIterBox<T>(ListNodeBox<T>);

impl<T> Iterator for IntoIterBox<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
}

struct IterBox<'a, T> {
    next: Option<&'a NodeBox<T>>,
}

impl<'a, T> Iterator for IterBox<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

struct IterMutBox<'a, T> {
    next: Option<&'a mut Box<NodeBox<T>>>,
}

impl<'a, T> Iterator for IterMutBox<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take()?;
        self.next = current.next.as_mut();
        Some(&mut current.data)
    }
}

struct ListNodeBox<T> {
    head: Link<T>,
}

impl<T> Default for ListNodeBox<T> {
    fn default() -> Self {
        Self {
            head: Default::default(),
        }
    }
}

impl<T> ListNodeBox<T> {
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn get_node_at(&self, position: usize) -> Option<&NodeBox<T>> {
        self.iter_nodes().nth(position)
    }

    fn new() -> Self {
        Self::default()
    }

    fn into_iter(self) -> IntoIterBox<T> {
        IntoIterBox(self)
    }
    
    fn iter(&self) -> IterBox<'_, T> {
        IterBox {
            next: self.head.as_deref(),
        }
    }
    
    fn iter_mut(&mut self) -> IterMutBox<'_, T> {
        IterMutBox {
            next: self.head.as_mut(),
        }
    }

    fn push_head(&mut self, data: T) {
        self.head = Some(Box::new(NodeBox {
            data,
            next: self.head.take(),
        }))
    }

    fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    fn join(mut self, mut other: Self) -> Self {
        if other.is_empty() {
            return self;
        }

        if self.is_empty() {
            return other;
        }

        let mut current = &mut self.head;
        while current.as_ref().expect("join error").next.is_some() {
            current = &mut current.as_mut().expect("join error").next;
        }
        if let Some(last_node) = current.as_mut() {
            last_node.next = other.head.take();
        }

        self
    }

    fn divide_at(&mut self, position: usize) -> Option<(Self, Self)> {
        if self.is_empty() {
            return None;
        }

        if position == 0 {
            return Some((Self::new(), std::mem::take(self)));
        }

        let mut current = &mut self.head;
        for _ in 0..position {
            match current.as_mut() {
                Some(node) => current = &mut node.next,
                None => return None,
            }
        }
        let head2 = current.take();

        let list1 = ListNodeBox {
            head: self.head.take(),
        };

        let list2 = ListNodeBox { head: head2 };

        Some((list1, list2))
    }

    fn append_at(&mut self, position: usize, data: T) {
        if position == 0 {
            self.push_head(data);
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..position {
            current = &mut current.as_mut().expect("append_at error").next;
        }

        let old_next = current.take();
        *current = Some(Box::new(NodeBox {
            data,
            next: old_next,
        }));
    }

    fn remove_at(&mut self, position: usize) {
        if self.is_empty() {
            return;
        }

        if position == 0 {
            self.pop_head();
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..position {
            current = &mut current.as_mut().expect("remove_at error").next;
        }

        if let Some(node) = current.as_mut() {
            *current = node.next.take();
        }
    }

    fn iter_nodes(&self) -> NodeIter<'_, T> {
        NodeIter {
            next: self.head.as_deref(),
        }
    }
}

struct NodeIter<'a, T> {
    next: Option<&'a NodeBox<T>>,
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a NodeBox<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            node
        })
    }
}
fn main() {
    let mut list = ListNodeBox::new();
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

#[test]
fn test_join() {
    let mut a = ListNodeBox::new();
    a.push_head(1);
    a.push_head(2);

    let mut b = ListNodeBox::new();
    b.push_head(3);
    b.push_head(4);

    let joined = a.join(b);

    let values: Vec<_> = joined.iter_nodes().map(|node| node.data).collect();
    assert_eq!(values, vec![2, 1, 4, 3]);
}

#[test]
fn test_divide_at() {
    let mut list = ListNodeBox::new();
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
    let mut list = ListNodeBox::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);

    list.append_at(1, 9);
    let collected: Vec<_> = list.iter_nodes().map(|node| node.data).collect();
    assert_eq!(collected, vec![3, 9, 2, 1]);
}

#[test]
fn test_remove_at() {
    let mut list = ListNodeBox::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    list.push_head(4);

    list.remove_at(3);
    let collected: Vec<_> = list.iter_nodes().map(|node| node.data).collect();
    assert_eq!(collected, vec![4, 3, 2]);
}
