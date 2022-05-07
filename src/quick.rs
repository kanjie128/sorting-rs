pub struct QuickSort;
use super::Sorter;

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        if slice.is_empty() {
            return;
        }
        let mut guard = 0usize;
        let mut start = 0usize;
        let mut end = slice.len() - 1;
        loop {
            while end >= start && end > 0 {
                if slice[end] >= slice[guard] {
                    end -= 1;
                } else {
                    slice.swap(guard, end);
                    guard = end;
                    end -= 1;
                    break;
                }
            }
            while end >= start {
                if slice[start] <= slice[guard] {
                    start += 1;
                } else {
                    slice.swap(start, guard);
                    guard = start;
                    start += 1;
                    break;
                }
            }
            if end < start {
                break;
            }
        }
        Self::sort(&mut slice[..guard]);
        Self::sort(&mut slice[(guard + 1)..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quick_sort_works() {
        let mut s = [7, 8, 2, 3, 9, 1, 4, 3, 5, 6, 1];
        QuickSort::sort(&mut s);
        assert_eq!(s, [1, 1, 2, 3, 3, 4, 5, 6, 7, 8, 9]);
        let mut s = [1];
        QuickSort::sort(&mut s);
        assert_eq!(s, [1]);
    }
}
