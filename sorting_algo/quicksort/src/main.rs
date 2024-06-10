fn partition(v: &mut [i32], p: usize, r: usize) -> usize {
    let x = v[r];
    let mut i = p;
    for j in p..r {
        if v[j] <= x {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, r);
    return i;
}

fn quicksort(v: &mut [i32], p: usize, r: usize) {
    if p < r {
        let q = partition(v, p, r);
        quicksort(v, p, q - 1);
        quicksort(v, q + 1, r);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![9, 3, 6, 1, -2, 44, 0];
    let p: usize = 0;
    let r: usize = v.len() - 1;
    quicksort(&mut v, p, r);

    for number in v {
        println!("{} ", number);
    }
}
