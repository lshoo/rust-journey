use std::cmp::Ordering;

pub fn binary_search<T: Ord>(data: &[T], desired: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = data.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        // let mid = (low + high) / 2;

        match data[mid].cmp(desired) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}
