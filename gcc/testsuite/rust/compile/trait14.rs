/* { dg-output "parent\r*\nchild\r*\n" } */

// Testing diamond problem with supertraits

extern "C" {
    fn printf(s: *const i8, ...);
}
 
struct Foo {
    my_int: u32,
}

trait GrandParent {
    fn grandparent(&self);
}

trait Parent1 : GrandParent {
    fn parent1(&self);
}

trait Parent2 : GrandParent {
    fn parent2(&self);
}

trait UhOh : Parent1+Parent2 {
    fn child(&self);
}

impl GrandParent for Foo {
    fn grandparent(&self) { let _ = self; }
}

impl Parent1 for Foo {
    fn parent1(&self) { let _ = self; }
}

impl Parent2 for Foo {
    fn parent2(&self) { let _ = self; }
}

impl UhOh for Foo {
    fn child(&self) {
        let _ = self;
    }
}

pub fn main() {
    let a = Foo{ my_int: 0xf00dfeed};
    let b: &dyn UhOh = &a;

    b.parent1();
    b.child();
}
