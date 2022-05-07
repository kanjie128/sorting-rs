use super::Sorter;
struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 1..slice.len() {
            let mut i = i;
            while i > 0 && slice[i] < slice[i - 1] {
                slice.swap(i, i - 1);
                i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion_sort_works() {
        let mut s = [2, 3, 1, 4, 3, 5];
        InsertionSort::sort(&mut s);
        assert_eq!(s, [1, 2, 3, 3, 4, 5]);
    }
}
