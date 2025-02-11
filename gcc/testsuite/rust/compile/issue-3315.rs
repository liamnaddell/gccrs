//You should be able to create a module of the same name as a builtin type

mod i32 {
}

fn main() -> isize {
    let i: i32 = 0 as i32;
    i as isize
}
