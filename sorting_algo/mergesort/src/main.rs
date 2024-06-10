fn merge(v: &mut Vec<i32>, p: usize, q: usize, r: usize) {
    let n: usize = q - p + 1;
    let m: usize = r - q;

    let mut left: Vec<i32> = Vec::with_capacity(n);
    let mut right: Vec<i32> = Vec::with_capacity(m);

    for i in 0..n {
        left.push(v[p + i]);
    }
    for j in 0..m {
        right.push(v[q + 1 + j]);
    }
    left.push(i32::MAX);
    right.push(i32::MAX);

    let mut i: usize = 0;
    let mut j: usize = 0;

    for k in p..=r {
        if left[i] <= right[j] {
            v[k] = left[i];
            i += 1;
        } else {
            v[k] = right[j];
            j += 1;
        }
    }
}

fn mergesort(v: &mut Vec<i32>, p: usize, r: usize) {
    if p < r {
        let q: usize = (p + r) / 2; //floor

        mergesort(v, p, q);
        mergesort(v, q + 1, r);
        merge(v, p, q, r);
    }
}

fn main() {
    let mut v: Vec<i32> = vec![7, 3, 9, 4, 5, 1, -23, 2];
    let p: usize = 0;
    let r: usize = v.len() - 1;
    mergesort(&mut v, p, r);

    for number in v {
        println!("{}", number);
    }
}
