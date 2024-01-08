fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let three = &v[2];
    println!("{}", three);
}

fn main_a() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(11);
    v.push(111);
    println!("{:?}", v);
}
