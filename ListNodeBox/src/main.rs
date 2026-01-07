// use std::marker::PhantomData;

// struct NodeBox<T> {
//     data: T,
//     next: Option<Box<NodeBox<T>>>
// }

// struct ListNodeBox<T> {
//     len: usize,
//     head: Option<Box<NodeBox<T>>>,
//     // tail: Option<Box<NodeBox<T>>> нельзя
// }

// struct IntoIter<T>(ListNodeBox<T>);

// struct Iter<'a, T> {
//     next: Option<Box<NodeBox<T>>>,
//     _marker: PhantomData<&'a T>
// }

// struct IterMut<'a, T> {
//     next: Option<Box<NodeBox<T>>>,
//     _marker: PhantomData<&'a mut  T>
// }

// impl<T> ListNodeBox<T> {
//     fn into_iter(self) -> IntoIter<T> {
//         IntoIter(self)
//     }

//     fn new() -> Self {
//         Self { len: 0, head: None }
//     }

//     fn push_back(&mut self, data: T) {
//         let new_node = NodeBox {
//             data,
//             next: None,
//         };

//         if let Some(ref mut head) = self.head {
//             let mut current = head;
//             while current.next.is_some() {
//                 current = current.next.as_mut().unwrap();
//             }
//             current.next = Some(Box::new(new_node));
//         } else {
//             self.head = Some(Box::new(new_node));
//         }

//         self.len += 1;
//     }

//     fn pop_back(&mut self) {
//         if self.len == 0 {
//             return;
//         }

//         if self.len == 1 {
//             self.head = None;
//             self.len = 0;
//             return;
//         }

//         let mut current = self.head.as_mut().unwrap();
//         for _ in 0..self.len-2 {
//             current = current.next.as_mut().unwrap();
//         }

//         current.next = None;
//         self.len -= 1;
//     }

//     fn join(&mut self, other: &mut Self) {
//         if other.len == 0 {
//             return;
//         }

//         if self.len == 0 {
//             self.len = other.len;
//             self.head = other.head.take();
//             other.len = 0;
//             return;
//         }

//         let mut current = self.head.as_mut().unwrap();
//         while current.next.is_some() {
//             current = current.next.as_mut().unwrap();
//         }
//         current.next = other.head.take();
//         self.len += other.len;
//         other.len = 0;
//     }

//     fn divide_at(&mut self, position: usize) -> Option<(Self, Self)>{
//         if self.len == 0{
//             return None;
//         } 
        
//         if position == 0 {
//             let second_list = Self {
//                 len: self.len,
//                 head: self.head.take(),
//             };
//             self.len = 0;
//             return Some((Self::new(), second_list));
//         }

//         if position >= self.len - 1 {
//             let first_list = Self { len: self
//                 .len, head: self.head.take() };
//             self.len = 0;
//             return Some((first_list, Self::new()));
//         }

//         let mut current = self.head.as_mut().unwrap();
//         for _ in 0..(position-1) {
//             current = current.next.as_mut().unwrap();
//         }
//         let head2 = current.next.take();/////////////////////////////////
//         let list1 = Self {
//             len: position,
//             head: self.head.take(),
//         };
//         let list2 = Self {
//             len: self.len - position,
//             head: head2,
//         };

//         self.len = 0;
//         Some((list1, list2))
//     }

//     fn append_at(&mut self, position: usize, data: T) {
//         let mut current = self.head.as_mut().unwrap();
//         for _ in 0..(position-1) {
//             current = current.next.as_mut().unwrap();
//         }

//         let new_node = NodeBox {
//             data,
//             next: current.next.take(),
//         };
//         current.next = Some(Box::new(new_node));
//         self.len += 1;
//     }

//     fn remove_at(&mut self, position: usize) {
//         let mut current = self.head.as_mut().unwrap();
//         for _ in 0..(position-1) {
//             current = current.next.as_mut().unwrap();
//         }
//         let node_to_remove = current.next.take().unwrap();
//         current.next = node_to_remove.next;
//         self.len -= 1;
//     }

//     // fn make_cycle_at(&mut self, position: usize) {
//     //     if self.len == 0 || position >= self.len {
//     //         return;
//     //     }
//     //     let mut current = self.head.as_mut().unwrap();
//     //     for _ in 0..(position-1) {
//     //         current = current.next.as_mut().unwrap();
//     //     }
//     // }

//     fn has_cycle(&self) -> bool {
//         if self.len == 0 {
//             return false;
//         }

//         let mut slow = self.head.as_ref();
//         let mut fast = self.head.as_ref();

//         while slow.is_some() && fast.is_some() {
//             slow = slow.and_then(|node| node.next.as_ref());

//             fast = fast.and_then(|node| node.next.as_ref());
//             fast = fast.and_then(|node| node.next.as_ref());

//             if let (Some(s), Some(f)) = (slow, fast) {
//                 if std::ptr::eq(s, f) {
//                     return true;
//                 }
//             }
//         }
//         false
//     }
// }

// impl<T> Drop for ListNodeBox<T>{
//     fn drop(&mut self) {
//         let mut current = self.head.take();
//         while let Some(mut node) = current {
//             current = node.next.take();
//         }
//         self.len = 0;
//     }
// }

// fn main() {
// }
