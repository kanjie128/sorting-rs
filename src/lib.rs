mod bubble;
mod insertion;
mod quick;
mod selection;
trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSort;
    impl Sorter for StdSort {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn std_sort_works() {
        let mut s = [6, 2, 4, 3, 1, 8];
        // sort::<_, StdSort>(&mut s);
        StdSort::sort(&mut s);
        assert_eq!(s, [1, 2, 3, 4, 6, 8]);
    }
}
