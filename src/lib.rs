#[must_use = "Iterators are lazy and must be consumed!"]
#[derive(Clone)]
pub struct GroupIterator<I, F>
where
    I: Clone + std::iter::Iterator,
    F: Fn(I::Item, I::Item) -> bool,
{
    iter: I,
    group_func: F,
}

impl<I, F> GroupIterator<I, F>
where
    I: Clone + std::iter::Iterator,
    F: Fn(I::Item, I::Item) -> bool,
{
    pub fn new(iter: I, group_func: F) -> Self {
        Self { iter, group_func }
    }
}

impl<I, F> Iterator for GroupIterator<I, F>
where
    I: Iterator + Clone,
    F: Fn(I::Item, I::Item) -> bool,
{
    // type Item = <I as Iterator>::Item;
    type Item = std::iter::Take<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let orig_iter = self.iter.clone();
        let mut cur = self.iter.clone();
        let mut next_cur = self.iter.clone();
        next_cur.next();
        let mut count = 0usize;

        loop {
            let value = cur.next();
            let value_next = next_cur.next();

            match (value, value_next) {
                (Some(a), Some(b)) => {
                    if (self.group_func)(a, b) {
                        count += 1;
                    } else {
                        self.iter = cur;
                        return Some(orig_iter.take(count + 1));
                    }
                }
                (Some(_), None) => {
                    self.iter = cur;
                    return Some(orig_iter.take(count + 1));
                }
                _ => return None,
            }
        }
    }
}

trait IteratorExt: Iterator + Clone {
    fn group_adjacent<F: Fn(Self::Item, Self::Item) -> bool>(
        self,
        group_func: F,
    ) -> GroupIterator<Self, F> {
        GroupIterator::new(self, group_func)
    }
}

impl<I: Iterator + Clone> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use super::*;
    #[test]
    fn map_and_filter_test() {
        let a = vec![1, 2, 3, 4, 0, 2, 3, 2, 1, 2, 3];

        let b = a.iter().map(|x| *x * 2 + 1).filter(|x| *x > 5);

        let c = b.collect::<Vec<_>>();

        assert_eq!(c, vec![7, 9, 7, 7]);
    }

    #[test]
    fn take_test() {
        let a = vec![1, 2, 3, 4, 0, 2, 3, 2, 1, 2, 3];

        let b = a.iter().take(3);

        let c = b.collect::<Vec<_>>();

        assert_eq!(c, vec![&1, &2, &3]);
    }

    #[test]
    fn group_iterator_test() {
        let a = vec![1, 2, 3, 4, 0, 2, 3, 2, 1, 2, 3];

        let b = a.iter();

        let c = GroupIterator::new(b, |x, y| x <= y);

        let d = c.map(|x| x.collect::<Vec<_>>()).collect::<Vec<_>>();

        assert_eq!(
            d,
            vec![
                vec![&1, &2, &3, &4],
                vec![&0, &2, &3],
                vec![&2],
                vec![&1, &2, &3]
            ]
        );
    }

    #[test]
    fn group_iterator_test_linked_list() {
        // let a = vec![1, 2, 3, 4, 0, 2, 3, 2, 1, 2, 3];
        let mut a: LinkedList<i32> = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        a.push_back(3);
        a.push_back(4);
        a.push_back(0);
        a.push_back(2);
        a.push_back(3);
        a.push_back(2);
        a.push_back(1);
        a.push_back(2);
        a.push_back(3);

        let b = a.iter();

        let c = GroupIterator::new(b, |x, y| x <= y);

        let d = c.map(|x| x.collect::<Vec<_>>()).collect::<Vec<_>>();

        assert_eq!(
            d,
            vec![
                vec![&1, &2, &3, &4],
                vec![&0, &2, &3],
                vec![&2],
                vec![&1, &2, &3]
            ]
        );
    }

    #[test]
    fn group_iterator_test_reversed() {
        let a = vec![1, 2, 3, 4, 0, 2, 3, 2, 1, 2, 3];

        let b = a.iter().rev();

        let c = GroupIterator::new(b, |x, y| x <= y);

        let d = c.map(|x| x.collect::<Vec<_>>()).collect::<Vec<_>>();

        assert_eq!(
            d,
            vec![
                vec![&3],
                vec![&2],
                vec![&1, &2, &3],
                vec![&2],
                vec![&0, &4],
                vec![&3],
                vec![&2],
                vec![&1],
            ]
        );
    }

    #[test]
    fn group_iterator_test_2() {
        let a = vec![1, 2, 3, 4, 0, 2, 3, 2, 1, 2, 3];

        let b = a
            .iter()
            .group_adjacent(|x, y| x <= y)
            .map(|x| x.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(
            b,
            vec![
                vec![&1, &2, &3, &4],
                vec![&0, &2, &3],
                vec![&2],
                vec![&1, &2, &3]
            ]
        );
    }

    #[test]
    fn group_iterator_test_3() {
        let a = vec![1, 2, 2, 3, 1, 2, 0, 4, 5, 2];

        let b = a
            .iter()
            .group_adjacent(|x, y| x <= y)
            .map(|x| x.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(
            b,
            vec![
                vec![&1, &2, &2, &3],
                vec![&1, &2],
                vec![&0, &4, &5],
                vec![&2]
            ]
        );
    }

    #[test]
    fn t() {
        let a = vec![1, 2, 3];

        let b = a
            .iter()
            .cycle()
            .take(6)
            .group_adjacent(|x, y| x <= y)
            .map(|x| x.collect::<Vec<_>>())
            .collect::<Vec<_>>();

        assert_eq!(b, vec![vec![&1, &2, &3], vec![&1, &2, &3]]);
    }
}
