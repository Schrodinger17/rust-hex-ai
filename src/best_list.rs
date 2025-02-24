#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct BestList<T: PartialOrd + Clone> {
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

    pub fn get(&self) -> Vec<T> {
        self.list.clone()
    }

    pub fn add(&mut self, item: T) {
        let cmp = |a: &T, b: &T| {
            if !self.rev {
                a > b
            } else {
                a < b
            }
        };

        if self.list.is_empty() {
            self.list.push(item);
            return;
        }

        let capacity = self.list.capacity();
        if self.list.capacity() != self.list.len() {
            self.list.push(item);
        } else {
            self.list[capacity - 1] = item;
        }
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

        assert_eq!(max_values.get(), vec![10, 9, 8]);
    }

    #[test]
    fn from_vec() {
        let values = vec![10, 2, 5, 4, 5, 8, 7, 8, 9];

        let max_values = BestList::from_vec(&values, 3);

        assert_eq!(values.get(0), Some(&10));
        assert_eq!(max_values.get(), vec![10, 9, 8]);
    }

    #[test]
    fn from_iter() {
        let values = vec![10, 2, 5, 4, 5, 8, 7, 8, 9];

        let max_values = BestList::from_iter(values.iter().cloned(), 3);

        assert_eq!(max_values.get(), vec![10, 9, 8]);
    }

    #[test]
    #[ignore = "benchmark"]
    fn time_comparison() {
        let size = 100000;
        let bests = 10;

        let mut list = vec![0; size];
        // fill with random values
        for e in list.iter_mut() {
            *e = rand::random::<i32>() % 100000;
        }

        // Best list
        let start = std::time::Instant::now();
        let mut best_list: BestList<i32> = BestList::new(bests);

        for e in list.iter() {
            best_list.add(*e);
        }
        let duration = start.elapsed();
        println!("BestList add_max: {:?}", duration);
        println!("BestList: {:?}", best_list.get());

        // Sort
        let start = std::time::Instant::now();

        list.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let best_list = list[0..bests].to_vec();

        let duration = start.elapsed();
        println!("Sort: {:?}", duration);
        println!("BestList: {:?}", best_list);
    }
}
