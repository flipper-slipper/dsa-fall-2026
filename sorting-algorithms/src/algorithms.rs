/// Sorts by repeatedly swapping the smallest remaining element into the next position.
pub fn selection_sort<T: Ord>(data: &mut [T]) {
    for i in 0..data.len() {
        let mut min = i;
        for j in (i + 1)..data.len() {
            if data[j] < data[min] {
                min = j;
            }
        }
        data.swap(i, min);
    }
}

/// Sorts by swapping each element backward until it reaches its place in the sorted prefix.
pub fn insertion_sort<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sorts by partitioning around a pivot and recursing on each side.
pub fn quick_sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    let p = partition(data);
    let (left, right) = data.split_at_mut(p);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

/// Helper method for quick sort that utilizes lumoto partition.
fn partition<T: Ord>(data: &mut [T]) -> usize {
    let last = data.len() - 1;
    let mut i = 0;
    for j in 0..last {
        if data[j] < data[last] {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, last);
    i
}

/// Sorts by recursively sorting each half and merging the results.
pub fn merge_sort<T: Ord + Clone>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    let mid = data.len() / 2;
    merge_sort(&mut data[..mid]);
    merge_sort(&mut data[mid..]);
    merge(data, mid);
}

/// Helper function for merge sort that merges the halves.
fn merge<T: Ord + Clone>(data: &mut [T], mid: usize) {
    let mut merged = Vec::with_capacity(data.len());
    let (mut i, mut j) = (0, mid);
    while i < mid && j < data.len() {
        if data[i] <= data[j] {
            merged.push(data[i].clone());
            i += 1;
        } else {
            merged.push(data[j].clone());
            j += 1;
        }
    }
    merged.extend_from_slice(&data[i..mid]);
    merged.extend_from_slice(&data[j..]);
    data.clone_from_slice(&merged);
}
