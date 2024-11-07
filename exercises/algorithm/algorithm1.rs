use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 合并两个链表
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        // 如果 list_a 为空，直接返回 list_b
        if list_a.start.is_none() {
            return list_b;
        }

        // 如果 list_b 为空，直接返回 list_a
        if list_b.start.is_none() {
            return list_a;
        }

        // 合并链表：将 list_a 的 end 连接到 list_b 的 start
        if let Some(end_a) = list_a.end {
            unsafe { (*end_a.as_ptr()).next = list_b.start };
        }

        LinkedList {
            start: list_a.start, // list_a 的 start 保持不变
            end: list_b.end,     // 新的 end 来自 list_b
            length: list_a.length + list_b.length, // 更新总长度
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut current = self.start;
        let mut result = String::new();
        while let Some(node_ptr) = current {
            unsafe {
                result.push_str(&format!("{} -> ", (*node_ptr.as_ptr()).val));
                current = (*node_ptr.as_ptr()).next;
            }
        }
        write!(f, "{}", result.trim_end_matches(" -> "))
    }
}

