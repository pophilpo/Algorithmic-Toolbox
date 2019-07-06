use std::io;
fn quicksort(array: &mut Vec<u64>) {
    let mut stack = vec![];
    stack.push((0, array.len()));
    while let Some(top) = stack.pop() {
        let (start, end) = top;
        match partition(array, start, end) {
            None => continue,
            Some(pivot_idx) => {
                if pivot_idx > start {
                    stack.push((start, pivot_idx));
                }
                if end > (pivot_idx + 1) {
                    stack.push((pivot_idx + 1, end));
                }
            },
        }
    }
}

fn swap(array: &mut Vec<u64>, a: usize, b: usize) {
	let tmp = array[a];
	array[a] = array[b];
	array[b] = tmp;
}

fn scan_for_next_lt(array: &mut Vec<u64>, start: usize, pivot_idx: usize) -> usize {
    let mut i = start;
    let pivot = array[pivot_idx];
    while i > pivot_idx {
        if array[i] < pivot {
            return i
        }
        i -= 1;
    }
    i
}

fn scan_for_next_geq(array: &mut Vec<u64>, start: usize, pivot_idx: usize) -> usize {
    let mut i = start;
    let pivot = array[pivot_idx];
    while i < pivot_idx {
        if array[i] >= pivot {
            return i
        }
        i += 1;
    }
    i
}

fn partition(array: &mut Vec<u64>, start: usize, end: usize) -> Option<usize> {
    if end - start <= 1 {
        None
    } else if end - start == 2 {
        if array[start] > array[end-1] {
            swap(array, start, end-1);
        }
        None
    } else {
        let pivot_idx = end - 1;
        let mut lt_counter = 0;
        let pivot = array[pivot_idx];
        for i in start..pivot_idx {
            if array[i] < pivot {
                lt_counter += 1;
            }
        }
        let new_pivot_idx = start + lt_counter;
        swap(array, pivot_idx, new_pivot_idx);

        let mut top = end-1;
        let mut bottom = start;
        while bottom < new_pivot_idx && top > new_pivot_idx {
            top = scan_for_next_lt(array, top, new_pivot_idx);
            bottom = scan_for_next_geq(array, bottom, new_pivot_idx);
            swap(array, top, bottom);
        }
        return Some(new_pivot_idx)
    }
}

fn main(){
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read line");
    let mut data = data.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u64>>();
    quicksort(&mut data);

    let result: String = data.iter().map(|x| x.to_string() + " ").collect();
    println!("{}", result);


}