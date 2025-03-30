/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count+=1;
        self.items.push(value);
        let mut currentId=self.count;
        let mut parentId=self.parent_idx(currentId);
        while parentId!=0{
            if (self.comparator)(&self.items[parentId], &self.items[currentId]){
                let inter=&self.items[parentId];
                self.items[parentId]=self.items[currentId];
                self.items[currentId]=*inter;

                currentId=parentId;
                parentId=self.parent_idx(currentId);
            }else{
                break;
            }
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
        //TODO
		let leftChild=self.left_child_idx(idx); let rightChild=self.right_child_idx(idx);
        if leftChild > self.count && rightChild>self.count {
            return 0;
        }

        if rightChild<=self.count{
            if (self.comparator)(&self.items[leftChild], &self.items[rightChild]){
                return rightChild;
            }else{
                return leftChild;
            }
        }else{
            return leftChild;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.count==0{
            return None;
        }

        let result=self.items[1];
        self.count-=1;

        self.items[1]=self.items[self.count];
        self.items.pop();

        let mut currentId=1;

        while self.children_present(currentId){
            if self.right_child_idx(currentId)<=self.count{
                let smallIdx=self.smallest_child_idx(currentId);
                if (self.comparator)(&self.items[currentId],&self.items[smallIdx]){
                    let inter=&self.items[currentId];
                    self.items[currentId]=self.items[smallIdx];
                    self.items[smallIdx]=*inter;

                    currentId=smallIdx;
                }else{
                    break;
                }
            }else{
                if (self.comparator)(&self.items[currentId],&self.items[self.left_child_idx(currentId)]){
                    let inter=self.items[currentId];
                    self.items[currentId]=self.items[self.left_child_idx(currentId) as usize];
                    self.items[self.left_child_idx(currentId)]=inter;

                    currentId=self.left_child_idx(currentId);
                }
            }
        }

        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
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