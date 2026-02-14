// Индексированный список на векторе
// Задача: использовать вектор, и операиции реализовать на векторе, а не на связном списке

#![allow(unused)]

struct IndexedList<T> {
    data: Vec<Option<T>>,
    free_list: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T> IndexedList<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            free_list: Vec::new(),
            head: None,
            tail: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        let mut current = self.head;
        std::iter::from_fn(move || {
            let ind = current?;
            current = self.find_next_occupied(ind);
            Some(ind)
        })
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.indices().map(move |i| self.data[i].as_ref().expect("Index refers to empty slot"))
    }

    fn into_iter(self) -> IntoIter<T> {
        let indices: Vec<usize> = self.indices().collect();
        IntoIter {
            indices,
            data: self.data,
            pos: 0,
        }
    }

    fn check_invariants(&self) {
        for &ind in &self.free_list {
            assert!(ind < self.data.len(), "Free index out of bounds");
            assert!(self.data[ind].is_none(), "Free index is occupied");
        }

        if let Some(h) = self.head {
            assert!(h < self.data.len(), "Head out of bounds");
            assert!(self.data[h].is_some(), "Head points to free slot");
        }

        if let Some(t) = self.tail {
            assert!(t < self.data.len(), "Tail out of bounds");
            assert!(self.data[t].is_some(), "Tail points to free slot");
        }
    }

    fn allocate_index(&mut self) -> usize {
        if let Some(ind) = self.free_list.pop() {
            ind
        } else {
            self.data.push(None);
            self.data.len() - 1
        }
    }

    fn find_next_occupied(&self, start: usize) -> Option<usize> {
        for i in (start + 1)..self.data.len() {
            if self.data[i].is_some() {
                return Some(i);
            }
        }
        None
    }

    fn find_previous_occupied(&self, start: usize) -> Option<usize> {
        for i in (0..start).rev() {
            if self.data[i].is_some() {
                return Some(i);
            }
        }
        None
    }

    fn push_head(&mut self, value: T) -> usize {
        let index = self.allocate_index();
        self.data[index] = Some(value);

        if self.head.is_none() {
            self.head = Some(index);
            self.tail = Some(index);
        }

        index
    }

    fn push_tail(&mut self, value: T) -> usize {
        let index = self.allocate_index();
        self.data[index] = Some(value);

        if self.head.is_none() {
            self.head = Some(index);
            self.tail = Some(index);
        } else {
            self.tail = Some(index);
        }

        index
    }

    fn pop_head(&mut self) -> Option<T> {
        let head_index = self.head?;
        let value = self.data[head_index].take()?;

        self.free_list.push(head_index);
        self.head = self.find_next_occupied(head_index);

        if self.head.is_none() {
            self.tail = None;
        }

        Some(value)
    }

    fn pop_tail(&mut self) -> Option<T> {
        let tail_index = self.tail?;
        let value = self.data[tail_index].take()?;

        self.free_list.push(tail_index);
        self.tail = self.find_previous_occupied(tail_index);

        if self.tail.is_none() {
            self.head = None;
        }

        Some(value)
    }

    fn remove_at(&mut self, position: usize) -> Option<T> {
        if position == 0 {
            return self.pop_head();
        }

        let index = self.indices().nth(position)?;

        if index >= self.data.len() {
            return None;
        }

        let value = self.data[index].take()?;
        self.free_list.push(index);

        if Some(index) == self.head {
            self.head = self.find_next_occupied(index);
        }
        if Some(index) == self.tail {
            self.tail = self.find_previous_occupied(index);
        }

        Some(value)
    }

    fn join(mut self, mut other: Self) -> Self {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return self;
        }

        let mut result = IndexedList::new();

        while let Some(item) = self.pop_head() {
            result.push_tail(item);
        }

        while let Some(item) = other.pop_head() {
            result.push_tail(item);
        }
        result
    }

    fn divide_at(mut self, position: usize) -> (Self, Self) {
        let mut left = IndexedList::new();
        let mut right = IndexedList::new();

        let mut current_pos = 0;

        while let Some(item) = self.pop_head() {
            if current_pos < position {
                left.push_tail(item);
            } else {
                right.push_tail(item);
            }
            current_pos += 1;
        }

        (left, right)
    }

    fn append_at(&mut self, position: usize, value: T) {
        let len = self.iter().count();

        if position == 0 {
            self.push_head(value);
            return;
        }
        if position >= len {
            self.push_tail(value);
            return;
        }

        let indices: Vec<usize> = self.indices().collect();
        let mut items = Vec::with_capacity(len + 1);

        for &idx in indices.iter().take(position) {
            items.push(self.data[idx].take().expect("Expected to have a value"));
        }

        items.push(value);

        for &idx in indices.iter().skip(position) {
            items.push(self.data[idx].take().expect("Expected to have a value"));
        }

        self.data.clear();
        self.free_list.clear();
        self.head = None;
        self.tail = None;

        for item in items {
            self.push_tail(item);
        }
    }
}

struct IntoIter<T> {
    indices: Vec<usize>,
    data: Vec<Option<T>>,
    pos: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.indices.len() {
            return None;
        }
        let ind = self.indices[self.pos];
        self.pos += 1;
        self.data[ind].take()
    }
}

impl<T> FromIterator<T> for IndexedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = IndexedList::new();
        list.extend(iter);
        list
    }
}

impl<T> Extend<T> for IndexedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();

        if lower > 0 {
            self.data.reserve(lower)
        }

        for item in iter {
            self.push_tail(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = IndexedList::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        let values: Vec<i32> = list.iter().copied().collect();
        assert_eq!(values, vec![1, 2, 3]);

        list.pop_head();
        list.pop_head();
        let values2: Vec<i32> = list.iter().copied().collect();
        assert_eq!(values2, vec![3]);
    }

    #[test]
    fn test_push_tail() {
        let mut list = IndexedList::new();
        list.push_tail(1);
        list.push_tail(2);
        list.push_tail(3);

        let values: Vec<i32> = list.iter().copied().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_large_list_drop() {
        let mut list = IndexedList::new();
        list.extend(0..1_000_000);
    }

    #[test]
    fn test_join() {
        let mut a = IndexedList::new();
        a.push_head(1);
        a.push_head(2);

        let mut b = IndexedList::new();
        b.push_head(3);
        b.push_head(4);

        let joined = a.join(b);

        let values: Vec<_> = joined.iter().copied().collect();
        assert_eq!(values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_divide_at() {
        let mut list = IndexedList::new();
        list.push_tail(1);
        list.push_tail(2);
        list.push_tail(3);
        list.push_tail(4);
        list.push_tail(5);

        let (a, b) = list.divide_at(3);

        let left: Vec<_> = a.iter().copied().collect();
        let right: Vec<_> = b.iter().copied().collect();

        assert_eq!(left, vec![1, 2, 3]);
        assert_eq!(right, vec![4, 5]);
    }

    #[test]
    fn test_append_at() {
        let mut list = IndexedList::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        list.append_at(1, 9);
        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected, vec![1, 9, 2, 3]);
    }

    #[test]
    fn test_remove_at() {
        let mut list = IndexedList::new();
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);
        list.push_head(4);

        list.remove_at(3);
        let collected: Vec<_> = list.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_iter() {
        let list: IndexedList<i32> = (0..5).collect();
        let values: Vec<_> = list.iter().copied().collect();
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }
}

fn create_and_drop_large_list() {
    let mut list = IndexedList::new();
    list.extend(0..1_000_000);
}

fn main() {
    let mut list = IndexedList::new();
    list.push_head(1);
    list.push_head(2);
    list.push_head(3);
    let values: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values, vec![1, 2, 3]);

    list.pop_head();
    list.pop_head();
    let values2: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values2, vec![3]);
}
