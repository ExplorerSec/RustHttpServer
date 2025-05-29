#![allow(dead_code)] // 取消这个子模块中的 dead_code 警告 //#![allow(warnings)] 

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

// 仿优先队列的 Vec，相同权重的元素自动按先后顺序放到权重对应 Vec 中
struct PriorityVec<C: Ord + Copy + Hash, T> {
    idx_len: usize, // 索引权重数
    val_len: usize, // 实际内容数，即所有 Vec 内的元素数目和
    index_s: BinaryHeap<Reverse<C>>,
    content_s: HashMap<C, Vec<T>>,
}

impl<C: Ord + Copy + Hash, T> PriorityVec<C, T> {
    fn new() -> PriorityVec<C, T> {
        PriorityVec {
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
    use super::PriorityVec;

    #[test]
    fn priority_vec_empty_len_insert() {
        let mut pv = PriorityVec::<i128, String>::new();
        assert!(pv.is_empty() == true);
        assert!(pv.len_idx() == 0);
        assert!(pv.len_val() == 0);
        assert!(pv.peek() == None);
        assert!(pv.pop() == None);
        pv.insert(1, "Hello".to_string());
        assert!(pv.is_empty() == false);
        assert!(pv.len_idx() == 1);
        assert!(pv.len_val() == 1);
        pv.insert(2, "Tmp".to_string());
        assert!(pv.len_idx() == 2);
        assert!(pv.len_val() == 2);
        pv.insert(1, "W".to_string());
        assert!(pv.len_idx() == 2);
        assert!(pv.len_val() == 3);
    }
    #[test]
    fn priority_vec_insert_peek_pop() {
        let mut pv = PriorityVec::<usize, char>::new();
        pv.insert(5, 'a');
        pv.insert(7, 'b');
        pv.insert(5, 'c');
        pv.insert(6, 'd');
        assert!(pv.peek() == Some(&vec!('a', 'c')));
        assert!(pv.pop() == Some(vec!('a', 'c')));
        assert!(pv.pop() == Some(vec!('d')));
        assert!(pv.peek() == Some(&vec!('b')));
        assert!(pv.pop() == Some(vec!('b')));
        assert!(pv.pop() == None);
    }
}
