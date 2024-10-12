/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default+ std::fmt::Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+ std::fmt::Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        println!("len:");
        for i in self.items.iter() {
            print!("{:?} ", i);
        }
        println!("");

        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        // let mut topFatherIndex = self.parent_idx(self.count);
        // let mut downFatherIndex = self.count;
        // while (self.comparator)(&self.items[self.count], &self.items[topFatherIndex])&&topFatherIndex!= 1 {
        //     downFatherIndex = topFatherIndex;
        //     topFatherIndex = self.parent_idx(topFatherIndex);
        //     println!("find two father")
        // }

        // let mut mezaIndex = topFatherIndex;
        // while (self.comparator)(&self.items[mezaIndex], &self.items[self.count]) {
        //     mezaIndex+=1;
        //     println!("find myself")
        // }
        // for i in (mezaIndex..self.count).rev() {
        //     self.items.swap(i, i+1);
        //     println!("change myself")
        // }
        for i in self.items.iter() {
            print!("{:?} ", i);
        }
        println!("count+2={:?}",self.count+2);

        for i in (1..self.count).rev() {
            // println!("for");
            if (self.comparator)(&self.items[i], &self.items[i+1]) {
                // println!("not swap");
                break;
            }
            else {
                // println!("swap");
                self.items.swap(i, i+1);
            }
        }
        for i in self.items.iter() {
            print!("{:?} ", i);
        }
        println!("");
    }

    // fn heapify(&mut self, idx: usize) {
    //     //TODO
    //     let mut index = self.count;
    //     while (self.comparator)(&self.items[index], &self.items[self.parent_idx(index)]) {
    //         let parent_index = self.parent_idx(index);
    //         self.items.swap(index, parent_index);
    //         index = self.parent_idx(index);
    //     }
    // }

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
        //TODO
		let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if !self.children_present(right) {
            left
        } else if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord+ std::fmt::Debug,
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
    T: Default+ std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        println!("next:");
        for i in self.items.iter() {
            print!("{:?} ", i);
        }
        println!("");
        //TODO
        if self.is_empty() {
		    None
        } else {
            self.count -= 1;
            let result = self.items.remove(1);
            println!("remove {:?}", result);
            // self.heapify(1);
            Some(result)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+ std::fmt::Debug,
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