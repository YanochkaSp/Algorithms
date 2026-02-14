// Индексированный список
#![allow(unused)]

struct NodeIndex<T> {
    data: Option<T>,
    next: Option<usize>,
}

struct ListNodeIndex<T> {
    nodes: Vec<NodeIndex<T>>,
    head: Option<usize>,
    free_list: Vec<usize>, //вектор из свободных индексов
}

// Для collect() в create_and_drop_large_list()
impl<T> FromIterator<T> for ListNodeIndex<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = ListNodeIndex::new();
        list.extend(iter);
        list
    }
}

struct IntoIterIndex<T> {
    list: ListNodeIndex<T>,
}

impl<T> Iterator for IntoIterIndex<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_head()
    }
}

struct IterIndex<'a, T> {
    list: &'a ListNodeIndex<T>,
    current: Option<usize>,
}

struct SlowFastIter<'a, T> {
    nodes: &'a [NodeIndex<T>],
    slow: Option<usize>,
    fast: Option<usize>
}

impl<'a, T> SlowFastIter<'a, T> {
    fn new(nodes: &'a [NodeIndex<T>], head: Option<usize>) -> Self {
        Self { nodes, slow: head, fast: head }
    }
}

impl<'a, T> Iterator for SlowFastIter<'a, T> {
    type Item = (Option<usize>, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.slow = self.slow.and_then(|i| self.nodes[i].next);
        self.fast = self.fast.and_then(|i| self.nodes[i].next).and_then(|i| self.nodes[i].next);

        if self.slow.is_none() {
            return None;
        }

        Some((self.slow, self.fast))
    }
}

impl<'a, T> Iterator for IterIndex<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ind = self.current?;
        self.current = self.list.nodes[ind].next;
        Some(
            self.list.nodes[ind]
                .data
                .as_ref()
                .expect("Expected successful conversion"),
        )
    }
}

impl<T> Extend<T> for ListNodeIndex<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();

        if lower > 0 {
            self.reserve(lower);
        }

        for item in iter {
            self.push_head(item);
        }
    }
}

impl<T> ListNodeIndex<T> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
            free_list: Vec::new(),
        }
    }

    fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn get_index_at(&self, position: usize) -> Option<usize> {
        self.iter_nodes().nth(position)
    }

    fn iter_nodes(&self) -> impl Iterator<Item = usize> {
        let nodes = &self.nodes;
        let mut current = self.head;
        std::iter::from_fn(move || {
            let ind = current?;
            current = nodes[ind].next;
            Some(ind)   
        })         
    }

    fn into_iter(self) -> IntoIterIndex<T> {
        IntoIterIndex { list: self }
    }

    fn iter(&self) -> IterIndex<'_, T> {
        IterIndex {
            list: self,
            current: self.head,
        }
    }

    fn check_invariants(&self) {
        if self.has_cycle() {
            panic!("Cycle detected in linked list!");
        }
    }

    fn push_node(&mut self, data: T, next: Option<usize>) -> usize {
        let ind = {
            if let Some(ind) = self.free_list.pop() {
                ind
            } else {
                self.nodes.len()
            }
        }; // Получить новый индекс
        let node = NodeIndex {
            data: Some(data),
            next,
        };

        if ind >= self.nodes.len() {
            self.nodes.push(node);
        } else {
            self.nodes[ind] = node;
        }
        ind
    }

    fn push_head(&mut self, data: T) {
        let new_head_ind = self.push_node(data, self.head);
        self.head = Some(new_head_ind);
    }

    fn pop_head(&mut self) -> Option<T> {
        let head_ind = self.head?;
        let node_to_pop = &mut self.nodes[head_ind];
        self.head = node_to_pop.next;

        self.free_list.push(head_ind);
        node_to_pop.data.take()
    }

    fn join(mut self, mut other: Self) -> Self {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return self;
        }
        let last_ind = self
            .iter_nodes()
            .last()
            .expect("Expected to have last node");
        let offset = self.nodes.len();
        let other_len = other.nodes.len();

        self.nodes.extend(other.nodes);

        for i in 0..other_len {
            let global_idx = offset + i;
            if let Some(next) = self.nodes[global_idx].next {
                self.nodes[global_idx].next = Some(next + offset);
            }
        }

        self.nodes[last_ind].next = other.head.map(|h| h + offset);

        for idx in other.free_list {
            self.free_list.push(idx + offset);
        }
        self
    }

    fn append_at(&mut self, position: usize, data: T) {
        if position == 0 {
            self.push_head(data);
            return;
        }

        let prev_ind = self
            .get_index_at(position - 1)
            .expect("Position out of bounds");
        let next_of_prev = self.nodes[prev_ind].next;
        let new_ind = self.push_node(data, next_of_prev);
        self.nodes[prev_ind].next = Some(new_ind);
    }

    fn remove_at(&mut self, position: usize) {
        if position == 0 {
            self.pop_head();
            return;
        }

        let prev_ind = self
            .get_index_at(position - 1)
            .expect("Position out of bounds");
        let ind_to_remove = self.nodes[prev_ind].next.expect("Expected node to remove");
        self.nodes[prev_ind].next = self.nodes[ind_to_remove].next;
        self.free_list.push(ind_to_remove);
    }

    fn make_cycle_at(&mut self, position: usize) {
        if self.is_empty() {
            return;
        }

        let target_ind = self.get_index_at(position - 1).expect("Invalid position");
        let last_ind = self.iter_nodes().last().expect("Error: empty list");
        self.nodes[last_ind].next = Some(target_ind);
    }

    fn has_cycle(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        SlowFastIter::new(&self.nodes, self.head).any(|(slow, fast)| slow == fast)
    }

    fn divide_at(self, position: usize) -> (Self, Self) {
        let mut iter = self.into_iter();

        let left_items: Vec<T> = iter.by_ref().take(position).collect();
        let right_items: Vec<T> = iter.collect();

        let mut left = ListNodeIndex::new();
        for item in left_items.into_iter().rev() {
            left.push_head(item);
        }

        let mut right = ListNodeIndex::new();
        for item in right_items.into_iter().rev() {
            right.push_head(item);
        }

        (left, right)
    }
}

fn main() {
    let mut list = ListNodeIndex::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    let values: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values, vec![3, 2, 1]);

    list.pop_head();
    list.pop_head();
    let values2: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values2, vec![1]);
}

fn create_and_drop_large_list() {
    // (0..1_000_000).collect::<ListNodeIndex<i32>>();

    // или так
    let mut list = ListNodeIndex::new();
    list.extend(0..1_000_000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_list_drop() {
        create_and_drop_large_list();
    }

    #[test]
    fn test_join() {
        let mut a = ListNodeIndex::new();
        a.push_head(1);
        a.push_head(2);

        let mut b = ListNodeIndex::new();
        b.push_head(3);
        b.push_head(4);

        let joined = a.join(b);

        let values: Vec<_> = joined.iter().copied().collect();
        assert_eq!(values, vec![2, 1, 4, 3]);
    }

    #[test]
    fn test_divide_at() {
        let mut list = ListNodeIndex::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        list.push_head(4);
        list.push_head(5);

        let (a, b) = list.divide_at(3);

        let left: Vec<_> = a.iter().copied().collect();
        let right: Vec<_> = b.iter().copied().collect();

        assert_eq!(left, vec![5, 4, 3]);
        assert_eq!(right, vec![2, 1]);
    }

    #[test]
    fn test_append_at() {
        let mut list = ListNodeIndex::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        list.append_at(1, 9);
        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected, vec![3, 9, 2, 1]);
    }

    #[test]
    fn test_remove_at() {
        let mut list = ListNodeIndex::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        list.push_head(4);

        list.remove_at(3);
        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected, vec![4, 3, 2]);
    }

    #[test]
    fn test_make_cycle_at() {
        let mut list = ListNodeIndex::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        list.push_head(4);

        list.make_cycle_at(1);
        assert!(list.has_cycle());
    }
}
