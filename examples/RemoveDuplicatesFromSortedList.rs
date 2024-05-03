





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


    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(node) => {
                let mut pointer = node;
                
                while let Some(ref next_node) = pointer.next {
                    if pointer.val == next_node.val {
                        pointer.next = next_node.next;
                    }
                }

                Some(node)
            },
            None => None,
        }
        
    }

fn main() {
  println!("Hello World");
}
