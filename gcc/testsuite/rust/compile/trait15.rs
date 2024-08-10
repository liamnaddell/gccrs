/* { dg-output "parent\r*\nchild\r*\n" } */

// Testing multiple bounds on dyn obj.
// TODO: Already a test for this?

extern "C" {
    fn printf(s: *const i8, ...);
}
 
struct Foo {
    my_int: u32,
}

trait Parent1 {
    fn parent1(&self);
}

trait Parent2 {
    fn parent2(&self);
}

impl Parent1 for Foo {
    fn parent1(&self) { let _ = self; }
}

impl Parent2 for Foo {
    fn parent2(&self) { let _ = self; }
}

pub fn main() {
    let a = Foo{ my_int: 0xf00dfeed};
    let b: &dyn Parent1 + Parent2 = &a;

    b.parent1();
    b.parent2();
}
