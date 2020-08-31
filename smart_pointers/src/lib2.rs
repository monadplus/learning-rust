#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// **** RefCell<T> and the Interior Mutability Pattern ****
pub fn ref_cell() {
    // Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.

    /*
      Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different from a type like Box<T>? Recall the borrowing rules you learned in Chapter 4:

        - At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
        - References must always be valid.

      **With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime.**

      The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
    */

    /*
     Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

         - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
         - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
         - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
    */

    // **** Interior Mutability Pattern: A Mutable Borrow to an Immutable Value ****

    // there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code. Code outside the value’s methods would not be able to mutate the value.

    // **** A Use Case for Interior Mutability: Mock Objects ****

    // See Messenger trait and tests section.

    // **** Keeping Track of Borrows at Runtime with RefCell<T> ****

    // The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like regular references.

    // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active. Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active. When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one. Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.

    // **** Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T> ****

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.borrow_mut
            self.sent_messages.borrow_mut().push(String::from(message));

            //let mut one_borrow = self.sent_messages.borrow_mut();
            //let mut two_borrow = self.sent_messages.borrow_mut();

            //one_borrow.push(String::from(message));
            //two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
