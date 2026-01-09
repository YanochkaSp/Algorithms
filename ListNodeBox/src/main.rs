type Link<T> = Option<Box<NodeBox<T>>>;

struct NodeBox<T> {
    data: T,
    next: Link<T>,
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
    fn new() -> Self {
        Self::default()
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

    fn iter_nodes(&self) -> NodeIter<T> {
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
