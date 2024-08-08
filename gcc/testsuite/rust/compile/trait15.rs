/* { dg-output "parent\r*\nchild\r*\n" } */
// Testing multiple supertraits and calling supertrait methods

extern "C" {
    fn printf(s: *const i8, ...);
}
 
struct Foo {
    my_int: u32,
}

trait Parent {
    fn parent(&self) -> bool;
}

trait Child : Parent {
    fn child(&self);
}

impl Parent for Foo {
    fn parent(&self) -> bool {
        // Call supertrait method
        return true;
    }
}

impl Child for Foo {
    fn child(&self) {
        let _ = self;
    }
}

pub fn main() {
    let a = Foo{ my_int: 0xf00dfeed};
    let b: &dyn Child = &a;

    let c: &dyn Parent = b;

    c.parent();
}
