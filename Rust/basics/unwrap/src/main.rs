fn main() -> Result<(), String> {
    let res: Result<(), String> = Ok(());

    let unwrapped_res = res.unwrap();
    println!("{:?}", unwrapped_res);

    Ok(())
}

fn main_a() {
    let some_value: Option<i32> = Some(52);

    let val = some_value.unwrap();
    println!("{val}");

}
