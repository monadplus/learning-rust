use core::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // `-> &T` is also ok
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};

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

    // **** Implicit Deref Coercions with Functions and Methods ****

    // Deref coercion converts such a type into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns str.

    // Deref coercion makes it possible to call `hello` with a reference to a vaue of type `MyBox<String>
    // &MyBox<String> -> &String -> &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // hello(&(*m)[..]);

    // ***** How Deref Coercion Interacts with Mutability ****

    //Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

    /* Rust does deref coercion when it finds types and trait implementations in three cases:
        - From &T to &U when T: Deref<Target=U>
        - From &mut T to &mut U when T: DerefMut<Target=U>
        - From &mut T to &U when T: Deref<Target=U>
    */
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_trait() {
    // Customize what happens when a value is about to go out of scope.
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // Variables are dropped in the reverse order of their creation

    // **** Dropping a Value Early with std::mem::drop ****

    // One example is when using smart pointers that manage locks

    //  Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.
    // c.drop() // compilation error
    drop(c); // from Prelude
    println!("CustomSmartPointers dropped before the end of main.");

    // You also don’t have to worry about problems resulting from accidentally cleaning up values still in use: the ownership system that makes sure references are always valid also ensures that drop gets called only once when the value is no longer being used.
}

// *** Rc<T>, the Reference Counted Smart Pointer ***
pub fn rc() {
    // For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.
    //
    // To enable multiple ownership, Rust has a type called Rc<T>
    //
    // We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.
    //
    // Rc<T> is only for single thread scenarios.

    // Fails to compile.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // We could use a.clone() but for convention we don't use it.
    // Visually this helps to avoid mistaken it by a deep copy.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // The call to Rc::clone only increments the ref. count
    let c = Cons(4, Rc::clone(&a));

    // **** Cloning an Rc<T> Increases the Reference Count ****

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
