//! Functionalized
//!
//!   - **Date:** October 29, 2015
//!   - **Subject:** Functions, methods, closures, and function as arguments!
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e004.mp3
//!
//! <audio style="width: 100%" title="Functionalized" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e004.mp3"></audio>
//!
//! # Notes
//!
//! This week's episode covers the basics of all sorts of functions: normal
//! functions, methods, and closures.
//!
//! ## Closures
//!   - [An explanation (in Ruby) by Martin Fowler][notes-1]
//!   - [Rust book][notes-2]
//!   - [Rust by Example][notes-3]
//!   - ["What is a closure?" (Progammers Stack Exchange)][notes-4] -- the first
//!     answer is the best, but the second answer may be a helpful stepping
//!     stone for people just getting their heads around this and coming from
//!     OOP languages like C++ or Java (even though I disagree with the
//!     explanation in some ways).
//!   - ["What is a closure?" (Stack Overflow)][notes-5] -- careful, thorough
//!     answer using JavaScript as an example.
//!
//! [notes-1]: http://martinfowler.com/bliki/Lambda.html
//! [notes-2]: https://doc.rust-lang.org/book/closures.html
//! [notes-3]: http://rustbyexample.com/fn/closures.html
//! [notes-4]: http://programmers.stackexchange.com/questions/40454/what-is-a-closure
//! [notes-5]: http://stackoverflow.com/questions/36636/what-is-a-closure
//!
//! # Links
//!
//!  - [Exercism][link-1] (hat tip: [Lechindanier on GitHub][link-2])
//!  - [Rust Learning][link-3]
//!  - [Rust and Swift (viii)][link-4]
//!
//! [link-1]: http://exercism.io/languages/rust
//! [link-2]: https://github.com/Lechindianer
//! [link-3]: https://github.com/ctjhoa/rust-learning
//! [link-4]: http://www.chriskrycho.com/2015/rust-and-swift-viii.html
//!
//! # Follow/Support
//!
//!   - New Rustacean:
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!       + <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
//!       + [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
//!       + App.net: [@chriskrycho](https://alpha.app.net/chriskrycho)

/// Create a module so we can see public/private behavior.
///
/// We will discuss modules in more detail in the future.
mod struct_container {
    /// Shows how methods work. Elaborates only a little on the e001 examples.
    pub struct MethodDemonstrator {
        // Public data.
        pub an_int: i64,
        pub a_string: String,

        // Private data.
        /// A tuple holding a floating point value and a string slice. (We'll
        /// discuss string slices in a future episode.)
        a_tuple: (f64, String),
    }

    impl MethodDemonstrator {
        /// A standard constructor pattern.
        ///
        /// You've seen this before, in the e001 code!
        ///
        /// Note that Rust doesn't have constructors in the same sense as C++
        /// or Java: you can construct a `MethodDemonstrator` just as this
        /// function does somewhere else. Using `new` is a convenient
        /// convention, so you can just call `MethodDemonstrator::new()` to get
        /// an instance, rather than needing to worry about the details of the
        /// struct.
        ///
        /// This is particularly important because not all types are
        /// necessarily public; you may not be able to construct a given
        /// `struct` *correctly* if it has hidden types, especially computed
        /// properties, which should be initialized during its construction.
        pub fn new() -> MethodDemonstrator {
            MethodDemonstrator {
                an_int: 0,
                a_string: "Nothin' into nothing, divide the nothin'...".to_string(),
                a_tuple: (2.0, "Twice 1.0".to_string()),
            }
        }

        /// A standard struct instance method.
        ///
        /// Note that *instance* methods take a reference to `self` as the
        /// first argument. It *needs* to be a reference for normal methods,
        /// because if it isn't, the struct instance will be moved into the
        /// function---the method will own, not just borrow---the reference,
        /// and after the method call ends, the item will be destroyed.
        ///
        /// Of course, if you need to write a custom destructor for a more
        /// complex type, you now have a pretty good idea how to write the first
        /// argument to that method...
        pub fn method(&self) {
            println!(
                "The values of the object are: {:}, {:}, {:}, {:}",
                self.an_int, self.a_string, self.a_tuple.0, self.a_tuple.1
            );
        }

        /// A getter for data which is not publicly accessible in the type.
        ///
        /// If you try to access the tuple contents directly, e.g. with an
        /// instance of the struct *outside this module*, you will fail. (See
        /// the example in `demonstrate_methods`.)
        ///
        /// The data can be accessed by the struct itself, however, so you can
        /// get or set the data, as here.
        ///
        /// We use `clone` because we need to get not the items themselves
        /// (which we could otherwise only get as references) but their values;
        /// the `clone` method is from the `Clone` trait, which is available on
        /// many basic types in the system. Again, we will return to `trait`s in
        /// a later episode.
        pub fn get_hidden_data(&self) -> (f64, String) {
            self.a_tuple.clone()
        }
    }
}

/// Shows how to use both struct and instance methods.
///
/// Note that struct methods are called with the `::` syntax, which is the same
/// as the module syntax! We'll come back to this soon. Note as well that the
/// `.` syntax used for instance methods corresponds to the use of `self` (in
/// whatever form) in
pub fn demonstrate_methods() {
    // Just builds a struct instance as expected.
    let a_struct = struct_container::MethodDemonstrator::new();

    // Call a basic method using `.` notation (which supplies `self` as the
    // first argument, in the appropriate fashion for the method).
    a_struct.method();

    // This won't work: the `a_tuple` member is private.
    //
    //     println!("{:?}", a_struct.a_tuple);
    //
    // However, we can get at the data if the struct gives us access:
    println!("{:?}", a_struct.get_hidden_data());
}

/// Shows how to take a function as an argument.
///
/// Note that this involves specifying a *generic* (the `<F>` syntax), bounded
/// by a `trait` (the `where...` syntax), a concept to which we will return in
/// much more detail in a few episodes.
pub fn apply_function_to_i64<F>(a_number_function: F, the_number: i64) -> i64
where
    F: Fn(i64) -> i64,
{
    let result = a_number_function(the_number);
    println!("{:?}", result);
    result
}

/// Show how to call a function with a function as an argument.
///
/// Both normal functions and closures can be passed as arguments to functions
/// which accept functions as arguments, as long as their type definition
/// matches the requirements of the destination function.
pub fn demonstrate_function_arguments() {
    /// Implements the signature required for `apply_function_to_i64`.
    ///
    /// Note that this is a nested function definition! It is unavailable
    /// outside the `demonstrate_function_arguments` body.
    fn double(n: i64) -> i64 {
        n * 2
    }

    // You can pass a normally defined function.
    assert_eq!(apply_function_to_i64(double, 2), 4);

    // You can also pass a closure, defined inline or standalone.
    // Inline closure definition:
    assert_eq!(apply_function_to_i64(|n| n * n, 5), 25);

    // Standalone closure definition
    let cube = |n| n * n * n;
    assert_eq!(apply_function_to_i64(cube, 9), 729);

    // You cannot use a function which does not meet the definition of the
    // target function, however, so this won't compile:
}

/// Shows how closures can act on elements within their environment.
///
/// Closures are handy for more than merely a quick and easy way to write a
/// callback; they also serve as one possible way to hide information or
/// implementation details. While that's not as strictly necessary in Rust as it
/// is in e.g. JavaScript, it still has a great deal of utility, especially in
/// more functional programming styles.
pub fn demonstrate_closure_environment() {
    /// Returns a closure which has access to the internal contents of this
    /// function even after it goes out of scope.
    fn get_a_closure() -> Box<dyn Fn(f64) -> f64> {
        let x = 14.0;

        // Now we define a closure. I'll explain the bits with `move` and
        // `Box::new` here next week; for now, suffice it to say that they're
        // necessary for Rust's memory guarantees and scoping behavior.
        let do_with_captured_x = move |n| n * x;
        Box::new(do_with_captured_x)
    }

    // Now call the closure. Note that even though we're now in a scope where
    // the value of `x` defined doesn't exist (try `println!("{:}", x);` if you
    // want to verify this), the closure still has access to it.
    let the_closure = get_a_closure();
    assert_eq!(the_closure(2.0), 28.0);

    /// Calls whatever function you hand it with the value 14.0
    fn takes_a_closure_with_14<F>(f: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        f(14.0)
    }

    // Note that the closure can interact with *this* environment as well as the
    // items handed to it by the function which calls it, because the `y` term
    // is available in its surrounding scope.
    let y = 3.0;
    assert_eq!(takes_a_closure_with_14(|n| n * y), 42.0);
}

#[test]
fn demonstrate() {
    demonstrate_function_arguments();
    demonstrate_methods();
    demonstrate_closure_environment();
}
