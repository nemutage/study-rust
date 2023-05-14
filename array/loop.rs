fn main() {
    let mut array: [u8; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    for x in array {
        print!("{} ", x);
        print_type(x);
        println!();
    }

    for x in &array {
        print!("{} ", x);
        print_type(x);
        println!();
    }
}

fn print_type<T>(_: T) {
    print!("{}", std::any::type_name::<T>());
}
