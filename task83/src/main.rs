use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match node {
        None => None,
        Some(node) => {
            
        }
    }
}
