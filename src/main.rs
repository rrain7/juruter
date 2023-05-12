use std::fmt::Display;

fn main() {
    println!("Hello, world!");
}


struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

// 第一个 T 代表类型参数
// `LinkedList<T>` 表示为类型<T>正在实现的方法
impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList{ head: None, length: 0 }
    }

    pub fn back_mut(&mut self) -> Option<Box<Node<T>>> {
        // some implementation
        todo!()
    }

    pub fn push_back(&mut self, val: T) {
        // some implementation
        todo!()
    }

    pub fn front(&mut self) -> Option<&Box<Node<T>>> {
        self.head.as_ref()
    }
}


impl <T: Display> LinkedList<T> {
    pub fn print(&mut self) {
        let mut curr = self.front();
        while let Some(node) = curr {
            println!("{}", node.value);
            curr = node.next.as_ref()
        }
    }
}