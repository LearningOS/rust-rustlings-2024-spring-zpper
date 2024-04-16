/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::f64::consts::PI;
use std::fmt::Debug;

pub struct Heap<T>
    where
        T: Default + Copy + Debug,
{
    count: usize,
    items: Vec<T>,
    index: usize,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
    where
        T: Default + Ord + Copy + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            index: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&mut self) -> usize {
        self.index = 0;
        self.count
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        // 上浮新添加的元素，以维护堆的性质
        self.swim(self.count - 1);
    }

    // 上浮操作
    fn swim(&mut self, mut index: usize) {
        while index > 0 && (self.comparator)(&self.items[index], &self.items[(index - 1) / 2]) {
            self.index = 0;
            self.items.swap(index, (index - 1) / 2);
            index = (index - 1) / 2;
        }
    }

    // 下沉操作
    fn sink(&mut self, mut index: usize) {
        let len = self.items.len();
        let mut child = 2 * index + 1;
        while child < len {
            let right = child + 1;
            if right < len && self.items[right] < self.items[child] {
                child = right;
            }
            if self.items[index] <= self.items[child] {
                break;
            }
            self.items.swap(index, child);
            index = child;
            child = 2 * index + 1;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        0
    }
}

impl<T> Heap<T>
    where
        T: Default + Ord + Copy + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
    where
        T: Default + Copy + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        println!("item {}=> {:?}", self.index, self.items);
        match self.items.get(self.index) {
            Some(v) => {
                self.index += 1;
                Some(*v)
            }
            None => {
                None
            }
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
        where
            T: Default + Ord + Copy + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
        where
            T: Default + Ord + Copy + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}