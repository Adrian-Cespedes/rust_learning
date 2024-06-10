fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}

fn max_heapify(arr: &mut [i32], i: usize, sz: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest: usize;

    if l < sz && arr[l] > arr[i] {
        largest = l;
    } else {
        largest = i;
    }

    if r < sz && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, largest, sz);
    }
}

fn build_max_heap(arr: &mut [i32]) {
    let sz = arr.len();
    for i in (0..sz / 2).rev() {
        max_heapify(arr, i, sz);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let mut sz = arr.len();
    build_max_heap(arr);
    for i in (1..sz).rev() {
        arr.swap(0, i);
        sz -= 1;
        max_heapify(arr, 0, sz);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![-3, 6, 1, 73, 2];
    heap_sort(&mut v);

    for e in v {
        println!("{}", e);
    }
}
