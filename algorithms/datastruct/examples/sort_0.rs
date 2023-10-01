fn main() {
    let mut s = vec![1, 4, 5, 2, 6, 9];
    bulle_sort2(&mut s);
    println!("bubble sort: {:?}", s);

    let mut s2 = vec![1, 4, 5, 2, 6, 9];
    let r = s2.len() - 1;
    quick_sort(&mut s2, 0, r);
    println!("quick sort: {:?}", s2)
}

#[allow(unused)]
fn bubble_sort(data: &mut [i32]) {
    let mut swapped = true;
    let mut len = data.len();
    while swapped && len > 1 {
        swapped = false;
        len -= 1;
        for i in 0..len {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

fn bulle_sort2(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }
    for len in 0..data.len() {
        for i in 0..data.len() - len - 1 {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1)
            }
        }
    }
}

fn quick_sort(a: &mut [i32], p: usize, r: usize) {
    if p >= r {
        return;
    }
    let q = partition(a, p, r);
    quick_sort(a, p, q - 1);
    quick_sort(a, q + 1, r)
}

fn quick_sort22(a: &mut [i32], p: usize, r: usize) {
    if p >= r {
        return;
    }
    let q = partition(a, p, r);
    quick_sort(a, p, q - 1)
}

fn partition(a: &mut [i32], p: usize, r: usize) -> usize {
    let pivot = a[r];
    let mut i = p;
    for j in p..r {
        if a[j] < pivot {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
}
