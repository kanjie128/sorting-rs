use super::Sorter;
struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut up = true;
        while up {
            up = false;
            for i in 0..slice.len() - 1 {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    up = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_sort_works() {
        let mut s = [2, 3, 1, 4, 3, 5];
        BubbleSort::sort(&mut s);
        assert_eq!(s, [1, 2, 3, 3, 4, 5]);
    }
}
