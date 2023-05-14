fn main() {
    let s = "hello";
    let r = s.as_bytes();
    println!("{:?}", r);

    let s = "This is a pen.";
    let r = s.contains("is");
    println!("{:?}", r);

    let s = "This is a pen.";
    let r = s.find("is");
    println!("{:?}", r);

    let s = "rust python go";
    let r = s.get(5..=10);
    println!("{:?}", r);
}
