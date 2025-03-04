#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BestList<T: PartialOrd + Clone> {
    list: Vec<T>,
    rev: bool,
}

impl<T: PartialOrd + Clone> BestList<T> {
    pub fn new(capacity: usize) -> BestList<T> {
        BestList {
            list: Vec::with_capacity(capacity),
            rev: false,
        }
    }

    pub fn new_rev(capacity: usize) -> BestList<T> {
        BestList {
            rev: true,
            ..BestList::new(capacity)
        }
    }

    pub fn get(&self) -> &Vec<T> {
        &self.list
    }

    pub fn add(&mut self, item: T) {
        let cmp = |a: &T, b: &T| {
            if !self.rev { a > b } else { a < b }
        };

        if self.list.is_empty() {
            self.list.push(item);
            return;
        }

        let capacity = self.list.capacity();
        if self.list.capacity() == self.list.len() {
            if cmp(&self.list[capacity - 1], &item) {
                return;
            }
            self.list[capacity - 1] = item;
        } else {
            self.list.push(item);
        }

        // Insertion sort
        let mut id = self.list.len() - 1;
        while id > 1 && cmp(&self.list[id], &self.list[id - 1]) {
            self.list.swap(id, id - 1);
            id -= 1;
        }
    }

    pub fn from_vec(vec: &[T], capacity: usize) -> BestList<T> {
        BestList::from_iter(vec.iter().cloned(), capacity)
    }

    pub fn from_iter<I>(iter: I, capacity: usize) -> BestList<T>
    where
        I: Iterator<Item = T>,
    {
        let mut res = BestList::new(capacity);
        for v in iter {
            res.add(v.clone());
        }
        res
    }
}

impl<T> From<BestList<T>> for Vec<T>
where
    T: Ord + Clone,
{
    fn from(val: BestList<T>) -> Self {
        val.list
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn add_test() {
        let mut max_values = BestList::new(3);
        let values = vec![10, 2, 5, 4, 5, 8, 7, 8, 9];

        for value in values {
            max_values.add(value);
        }

        assert_eq!(*max_values.get(), vec![10, 9, 8]);
    }

    #[test]
    fn from_vec() {
        let values = vec![10, 2, 5, 4, 5, 8, 7, 8, 9];

        let max_values = BestList::from_vec(&values, 3);

        assert_eq!(values.get(0), Some(&10));
        assert_eq!(*max_values.get(), vec![10, 9, 8]);
    }

    #[test]
    fn from_iter() {
        let values = vec![10, 2, 5, 4, 5, 8, 7, 8, 9];

        let max_values = BestList::from_iter(values.iter().cloned(), 3);

        assert_eq!(*max_values.get(), vec![10, 9, 8]);
    }
}
