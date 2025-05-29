#![allow(dead_code)] // 取消这个子模块中的 dead_code 警告 //#![allow(warnings)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;

// 优先队列，允许相同元素，权重相同时先入先出
pub struct PriorityQueue<C: Ord + Copy + Hash, T> {
    idx_len: usize, // 索引权重数
    val_len: usize, // 实际内容数，即所有 Vec 内的元素数目和
    index_s: BinaryHeap<Reverse<C>>,
    content_s: HashMap<C, VecDeque<T>>,
}

impl<C: Ord + Copy + Hash, T> PriorityQueue<C, T> {
    pub fn new() -> PriorityQueue<C, T> {
        PriorityQueue {
            idx_len: 0,
            val_len: 0,
            index_s: BinaryHeap::<Reverse<C>>::new(),
            content_s: HashMap::<C, VecDeque<T>>::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.idx_len == 0
    }
    pub fn len(&self) -> usize {
        self.val_len
    }
    pub fn insert(&mut self, weight: C, val: T) {
        // 测试发现 BinaryHeap 重复 push 会增加 len
        match self.content_s.get_mut(&weight) {
            Some(val_vec) => {
                val_vec.push_back(val);
            }
            None => {
                self.idx_len += 1;
                self.index_s.push(Reverse(weight));
                self.content_s.insert(weight, VecDeque::from([val]));
            }
        }
        self.val_len += 1;
    }
    pub fn peek(&self) -> Option<&T> {
        match self.index_s.peek() {
            None => None,
            Some(Reverse(weight)) => {
                let vec_deque = self.content_s.get(weight).unwrap();
                vec_deque.front()
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.index_s.peek() {
            None => None,
            Some(Reverse(weight)) => {
                let vec = self.content_s.get_mut(weight).unwrap();
                let val = vec.pop_front();
                self.val_len -= 1;

                if vec.is_empty() {
                    self.idx_len -= 1;
                    self.content_s.remove(weight);
                    self.index_s.pop();
                }

                val
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::PriorityQueue;
    #[test]
    fn priority_queue_empty_len_insert() {
        let mut pq = PriorityQueue::<usize, char>::new();
        assert_eq!(pq.is_empty(), true);
        assert_eq!(pq.len(), 0);
        assert_eq!(pq.peek(), None);
        pq.insert(2, 'a');
        pq.insert(3, 'b');
        pq.insert(2, 'c');
        pq.insert(1, 'd');
        assert_eq!(pq.len(), 4);
        assert_eq!(pq.pop(), Some('d'));
        assert_eq!(pq.pop(), Some('a'));
        assert_eq!(pq.peek(), Some(&'c'));
        assert_eq!(pq.pop(), Some('c'));
        assert_eq!(pq.len(), 1);
        assert_eq!(pq.pop(), Some('b'));
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.peek(), None);
        assert_eq!(pq.len(), 0);
    }
}
