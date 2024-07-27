
fn main() {
    let mut arr = vec![10, 7, 8, 9, 1, 5];
    let n = arr.len();
    quicksort(&mut arr, 0, n - 1);
    println!("Sorted array: {:?}", arr);
}

fn quicksort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);
        if pi > 0 {
            quicksort(arr, low, pi - 1);
        }
        quicksort(arr, pi + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}