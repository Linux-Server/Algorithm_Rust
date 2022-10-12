fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i = *i + 50;
        println!("{}", i);
    }

    println!("The oucome : {:?}", v);
}
