struct Test<'a> {
    message: &'a str,
}

impl<'a> Test<'a> {
    fn longest(&'a self, message: &'a str) -> &'a str {
        if self.message.len() < message.len() {
            message
        } else {
            self.message
        }
    }
}

fn main() {
    let test: Test = Test { message: "rust" };
    let result: &str = test.longest("go");
    println!("{}", result);
}
