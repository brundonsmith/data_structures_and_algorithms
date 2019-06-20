
#[cfg(test)]
mod sort_test {

    extern crate rand;
    use std::time::Instant;

    use crate::algorithms::sort::{SortFn,bubble_sort,insertion_sort,selection_sort,quick_sort,partition,merge_sort};
    
    /*
    const TEST_CASE_1: ([i32;6],[i32;6]) = 
        ([ 3, 7, 1, 3, 9, 6 ], 
         [ 1, 3, 3, 6, 7, 9 ]);

    const TEST_CASE_2: ([&str;6],[&str;6]) = 
        ([ "pear", "apple", "orange", "banana", "peach", "coconut" ], 
         [ "apple", "banana", "coconut", "orange", "peach", "pear" ]);

    let mut arr = TEST_CASE_1.0.clone();
    bubble_sort(&mut arr);
    println!("{:?} -> {:?}", TEST_CASE_1.0, arr);
    assert_eq!(arr, TEST_CASE_1.1);

    let mut arr = TEST_CASE_2.0.clone();
    bubble_sort(&mut arr);
    println!("{:?} -> {:?}", TEST_CASE_2.0, arr);
    assert_eq!(arr, TEST_CASE_2.1);
    */

    fn generate_test_case() -> Vec<i32> {
        (0..1000).into_iter()
            .map(|_| rand::random::<i32>() % 1000)
            .collect::<Vec<i32>>()
    }

    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i+1] {
                return false;
            }
        }
        return true;
    }

    fn test_sort(func: &SortFn<i32>, name: &str) {
        print!("    {}() | ", name);

        let start = Instant::now();

        for _ in 0..20 {
            let test = generate_test_case();
            let mut arr = test.clone();
            func(&mut arr);
            //println!("\n<- {:?} \n-> {:?}\n", test, arr);
            assert!(is_sorted(&arr));
        }

        print!("{} milliseconds  ... ", start.elapsed().as_millis());
    }


    #[test]
    fn bubble() {
        test_sort(&bubble_sort, "bubble_sort");
    }

    #[test]
    fn insertion() {
        test_sort(&insertion_sort, "insertion_sort");
    }

    #[test]
    fn selection() {
        test_sort(&selection_sort, "selection_sort");
    }

    #[test]
    fn quick() {
        test_sort(&quick_sort, "quick_sort");
    }

    #[test]
    fn part() {
        let mut arr = [3, 2, 7, 3, 9, 6];
        partition(&mut arr);
        assert_eq!(arr, [3, 2, 3, 7, 9, 6]);

        let mut arr = [1, 7, 9, 6];
        partition(&mut arr);
        assert_eq!(arr, [1, 7, 6, 9]);
    }

    #[test]
    fn merge() {
        test_sort(&merge_sort, "merge_sort");
    }
}