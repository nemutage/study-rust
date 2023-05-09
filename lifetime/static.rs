static mut STASH1: &i32 = &128;
static mut STASH2: &i32 = &12;

fn f(p: &'static i32) {
    unsafe {
        STASH2 = p;
    }
}

fn main() {
    unsafe {
        let p: &i32 = STASH1;
        f(p);
    }
}
