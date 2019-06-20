
pub type SortFn<T> = Fn(&mut [T]) -> ();



pub fn bubble_sort<T: Ord+Copy>(arr: &mut [T]) {
    let mut bubbled = true;

    while bubbled {
        bubbled = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                bubbled = true;
            }
        }
    }
}



pub fn insertion_sort<T: Ord+Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut focal_index = i;
        while focal_index > 0 && arr[focal_index] < arr[focal_index-1] {
            arr.swap(focal_index - 1, focal_index);
            focal_index -= 1;
        }
    }
}



pub fn selection_sort<T: Ord+Copy>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let focal_index = select_min(i, &arr);
        arr.swap(i, focal_index);
    }
}

fn select_min<T: Ord+Copy>(left_index: usize, arr: &[T]) -> usize {
    let mut min_index = left_index;

    for i in left_index+1..arr.len() {
        if arr[i] < arr[min_index] {
            min_index = i;
        }
    }

    return min_index;
}



pub fn quick_sort<T: Ord+Copy>(arr: &mut [T]) {
    let arr_len = arr.len();

    if arr_len <= 1 {
        return;
    } else {
        let pivot_index = partition(arr);
        quick_sort(&mut arr[0..pivot_index]);
        quick_sort(&mut arr[pivot_index..arr_len]);
    }
}

pub fn partition<T: Ord+Copy>(arr: &mut [T]) -> usize {
    let pivot = arr[arr.len() / 2];
    let mut lo = 0;
    let mut hi = arr.len() - 1;

    loop {
        while arr[lo] < pivot { 
            lo += 1 };
        while arr[hi] > pivot { 
            hi -= 1 };

        if lo >= hi { 
            return lo; 
        } else {
            arr.swap(lo, hi);

            if arr[lo] == pivot { lo += 1 }
            else if arr[hi] == pivot { hi -= 1 }
            else {
                lo += 1;
                hi -= 1;
            }
        }
    }
}



pub fn merge_sort<T: Ord+Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut slices = arr.split_at_mut(mid);

        if slices.0.len() == 0 || slices.1.len() == 0 {
            print!("");
        }

        merge_sort(&mut slices.0);
        merge_sort(&mut slices.1);
        merge(&mut slices.0, &mut slices.1);
    }
}

fn merge<T: Ord+Copy>(arr1: &mut [T], arr2: &mut [T]) {
    let mut buffer: Vec<T> = Vec::with_capacity(arr1.len() + arr2.len());
    
    let mut index_1 = 0;
    let mut index_2 = 0;

    // merge
    while index_1 < arr1.len() || index_2 < arr2.len() {

        let el_1 = if index_1 < arr1.len() {
            Some(arr1[index_1])
        } else {
            None
        };

        let el_2 = if index_2 < arr2.len() {
            Some(arr2[index_2])
        } else {
            None
        };

        let next_item = match el_1 {
            None => {
                index_2 += 1;
                el_2.unwrap()
            },
            Some(el_1_val) => match el_2 {
                None => {
                    index_1 += 1;
                    el_1_val
                },
                Some(el_2_val) =>
                    if el_1_val < el_2_val {
                        index_1 += 1;
                        el_1_val
                    } else {
                        index_2 += 1;
                        el_2_val
                    }
            }
        };

        buffer.push(next_item);
    }

    // copy from buffer
    for i in 0..buffer.len() {
        if i < arr1.len() {
            arr1[i] = buffer[i];
        } else {
            arr2[i - arr1.len()] = buffer[i];
        }
    }
}