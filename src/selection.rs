use super::Sorter;
struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let smallest = slice[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, k)| k)
                .map(|(i, _)| i)
                .unwrap();
            let smallest = smallest + i;
            slice.swap(i, smallest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_sort_works() {
        let mut s = [3, 2, 1, 4, 3, 5];
        SelectionSort::sort(&mut s);
        assert_eq!(s, [1, 2, 3, 3, 4, 5]);
    }
}
