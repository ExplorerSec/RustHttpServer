#![allow(dead_code)] // 取消这个子模块中的 dead_code 警告 //#![allow(warnings)] 

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

struct PriorityMap<C: Ord + Copy + Hash, T> {
    idx_len: usize, // 索引权重数
    val_len: usize, // 实际内容数，即所有 Vec 内的元素数目和
    index_s: BinaryHeap<Reverse<C>>,
    content_s: HashMap<C, Vec<T>>,
}

impl<C: Ord + Copy + Hash, T> PriorityMap<C, T> {
    fn new() -> PriorityMap<C, T> {
        PriorityMap {
            idx_len: 0,
            val_len: 0,
            index_s: BinaryHeap::<Reverse<C>>::new(),
            content_s: HashMap::<C, Vec<T>>::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.idx_len == 0
    }
    fn len_idx(&self) -> usize {
        self.idx_len
    }
    fn len_val(&self) -> usize {
        self.val_len
    }
    fn insert(&mut self, weight: C, val: T) {
        // 测试发现 BinaryHeap 重复 push 会增加 len
        match self.content_s.get_mut(&weight) {
            Some(val_vec) => {
                val_vec.push(val);
            }
            None => {
                self.idx_len += 1;
                self.index_s.push(Reverse(weight));
                self.content_s.insert(weight, vec![val]);
            }
        }
        self.val_len += 1;
    }
    fn peek(&self) -> Option<&Vec<T>> {
        match self.index_s.peek() {
            None => None,
            Some(Reverse(weight)) => self.content_s.get(weight),
        }
    }
    fn pop(&mut self) -> Option<Vec<T>> {
        match self.index_s.pop() {
            None => None,
            Some(Reverse(weight)) => {
                self.idx_len -= 1;
                let vec = self.content_s.remove(&weight).unwrap();
                self.val_len -= vec.len();
                Some(vec)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::PriorityMap;
    #[test]
    fn prioritymap_empty_len_insert() {
        let mut pm = PriorityMap::<i128, String>::new();
        assert!(pm.is_empty() == true);
        assert!(pm.len_idx() == 0);
        assert!(pm.len_val() == 0);
        pm.insert(1, "Hello".to_string());
        assert!(pm.is_empty() == false);
        assert!(pm.len_idx() == 1);
        assert!(pm.len_val() == 1);
        pm.insert(2, "Tmp".to_string());
        assert!(pm.len_idx() == 2);
        assert!(pm.len_val() == 2);
        pm.insert(1, "W".to_string());
        assert!(pm.len_idx() == 2);
        assert!(pm.len_val() == 3);
    }
    #[test]
    fn prioritymap_insert_peek_pop() {
        let mut pm = PriorityMap::<usize, char>::new();
        pm.insert(5, 'a');
        pm.insert(7, 'b');
        pm.insert(5, 'c');
        pm.insert(6, 'd');
        assert!(pm.peek() == Some(&vec!('a', 'c')));
        assert!(pm.pop()==Some(vec!('a', 'c')));
        assert!(pm.pop()==Some(vec!('d')));
        assert!(pm.peek()==Some(&vec!('b')));
        assert!(pm.pop()==Some(vec!('b')));
        assert!(pm.pop()==None);
    }
}
