fn find_larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn find_larger_in_vec<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for num in list {
        if largest < num {
            largest = num;
        }
    }
    largest
}

fn main() {
    let result_int = find_larger(5, 10);
    let result_float = find_larger(3.5, 2.0);

    let v = vec![1, 3, 5, 3, 2, 355];
    let result_int_in_vec = find_larger_in_vec(v);

    let v_float = vec![2.0, 1.0, 55.0, 23.0];
    let result_float_in_vec = find_larger_in_vec(v_float);

    println!("Larger integer: {}", result_int);
    println!("Larger float: {}", result_float);
    println!("Largest int in vec: {}", result_int_in_vec);
    println!("Largest int in vec: {}", result_float_in_vec);
}
