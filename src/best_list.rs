#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct BestList<T: PartialOrd + Clone> {
    size: usize,
    list: Vec<T>,
}

impl<T: PartialOrd + Clone> BestList<T> {
    #[allow(dead_code)]
    pub fn new(size: usize) -> BestList<T> {
        BestList {
            size,
            list: Vec::with_capacity(size),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self) -> Vec<T> {
        self.list.clone()
    }

    #[allow(dead_code)]
    pub fn add_max(&mut self, item: T) {
        if self.list.len() < self.size {
            self.list.push(item);
        } else {
            let mut min = 0;
            for i in 1..self.size {
                if self.list[i] < self.list[min] {
                    min = i;
                }
            }
            if item > self.list[min] {
                self.list[min] = item;
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_min(&mut self, item: T) {
        if self.list.len() < self.size {
            self.list.push(item);
        } else {
            let mut max = 0;
            for i in 1..self.size {
                if self.list[i] > self.list[max] {
                    max = i;
                }
            }
            if item < self.list[max] {
                self.list[max] = item;
            }
        }
    }

    pub fn sort(&mut self) {
        self.list.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }

    pub fn reversed_sort(&mut self) {
        self.list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut list_max = BestList::new(3);
        let mut list_min = BestList::new(3);
        let values = vec![10, 2, 5, 4, 5, 8, 7, 8, 9];

        for value in values {
            list_max.add_max(value);
            list_min.add_min(value);
        }

        assert_eq!(list_max.get(), vec![10, 9, 8]);
        assert_eq!(list_min.get(), vec![4, 2, 5]);
    }
}
