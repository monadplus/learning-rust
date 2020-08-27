use core::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // `-> &T` is also ok
        &self.0
    }
}

pub fn deref_trait() {
    // We’ll explore how implementing the Deref trait makes it possible for smart pointers to work in ways similar to references

    // Note: there’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our version will not store its data on the heap. We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.

    // A regular reference is a tpye of pointer.
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference

    // Using Box<T> like a reference
    let y = Box::new(x);
    assert_eq!(5, *y); // dereference

    // Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how smart pointers behave differently from references by default
    let b = MyBox::new(x);
    assert_eq!(5, *b); // Syntax sugar for: *(b.deref())
}
