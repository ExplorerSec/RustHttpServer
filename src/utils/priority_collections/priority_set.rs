#![allow(dead_code)] // 取消这个子模块中的 dead_code 警告 //#![allow(warnings)] 

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

// 集合型-优先队列，不允许相同权重的元素
struct PrioritySet<C: Ord + Copy + Hash, T> {
    index_s: BinaryHeap<Reverse<C>>,
    content_s: HashMap<C, T>,
}

impl<C: Ord + Copy + Hash, T> PrioritySet<C, T> {
    fn new() -> PrioritySet<C, T> {
        PrioritySet {
            index_s: BinaryHeap::<Reverse<C>>::new(),
            content_s: HashMap::<C, T>::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.index_s.is_empty()
    }
    fn len(&self) -> usize {
        self.index_s.len()
    }
    fn insert_or_update(&mut self, weight: C, val: T) {
        // BinaryHeap 重复 push 会增加 len，但 HashMap insert 是插入或更新，返回 None 表示插入
        if self.content_s.insert(weight, val).is_none() {
            self.index_s.push(Reverse(weight));
        }
    }
    fn insert_only(&mut self, weight: C, val: T) -> bool {
        match self.content_s.contains_key(&weight) {
            false => {
                self.index_s.push(Reverse(weight));
                self.content_s.insert(weight, val);
                true
            }
            true => false,
        }
    }
    fn update_only(&mut self, weight: C, val: T) -> bool {
        match self.content_s.get_mut(&weight) {
            None => false,
            Some(v) => {
                *v = val;
                true
            }
        }
    }
    fn peek(&self) -> Option<&T> {
        match self.index_s.peek() {
            None => None,
            Some(Reverse(weight)) => self.content_s.get(weight),
        }
    }
    fn pop(&mut self) -> Option<T> {
        match self.index_s.pop() {
            None => None,
            Some(Reverse(weight)) => self.content_s.remove(&weight),
        }
    }
}

#[cfg(test)]
mod test {
    use super::PrioritySet;
    #[test]
    fn priority_set_insert_or_update_pop_peek() {
        let mut ps = PrioritySet::<i32, char>::new();
        assert_eq!(ps.is_empty(), true);
        assert_eq!(ps.len(), 0);
        assert_eq!(ps.peek(), None);
        assert_eq!(ps.pop(), None);
        ps.insert_or_update(1, 'a');
        ps.insert_or_update(-1, 'b');
        ps.insert_or_update(4, 'c');
        ps.insert_or_update(1, 'd');
        assert_eq!(ps.is_empty(), false);
        assert_eq!(ps.len(), 3);
        assert_eq!(ps.pop(), Some('b'));
        assert_eq!(ps.pop(), Some('d'));
        assert_eq!(ps.pop(), Some('c'));
        assert_eq!(ps.pop(), None);
        assert_eq!(ps.peek(), None);
    }
    #[test]
    fn priority_set_only_insert_update() {
        let mut ps = PrioritySet::<usize, &'static str>::new();
        assert_eq!(ps.update_only(2, "temp"), false);
        assert_eq!(ps.peek(), None);
        assert_eq!(ps.insert_only(1, "123"), true);
        assert_eq!(ps.insert_only(1, "456"), false);
        assert_eq!(ps.peek(), Some(&"123"));
        assert_eq!(ps.update_only(4, "111"), false);
        assert_eq!(ps.update_only(1, "t4"), true);
        assert_eq!(ps.pop(), Some("t4"));
        assert_eq!(ps.pop(), None);
    }
}
