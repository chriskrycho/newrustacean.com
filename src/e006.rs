//! Modularize this!
//!
//!   - **Date:** November 16, 2015
//!   - **Subject:** Designing APIs, and using packages ("crates") and modules
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e006.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e006.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e006.ogg)
//!
//! <audio title="Modularize this!" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e006.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e006.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e006.ogg">
//! </audio>
//!
//! # Notes
//!
//! Today, we are talking about modules, packages, and APIs in Rust. Taking a
//! bit of a breather after some pretty hard material the last few weeks.
//!
//! For reference, the [Rust book][notes-1] section on [Crates and
//! Modules][notes-2] will be very helpful.
//!
//! [notes-1]: http://doc.rust-lang.org/book/
//! [notes-2]: http://doc.rust-lang.org/book/crates-and-modules.html
//!
//! ## Corrigenda
//!
//! I accidentally called this episode 5, instead of episode 6. *Whoops.*
//!
//! Just before the 15:00 mark, while discussing libraries, I referred to
//! "e006.md" when I meant to say "e006.rs". Slips of the tongue inspired by the
//! fact that Rust (delightfully) uses Markdown for its documentation.
//!
//! # Links
//!
//!   - [Roguelike in Rust][links-1]
//!   - [Yehuda Katz on Ruby FFI][links-2]
//!
//! [links-1]: http://jaredonline.svbtle.com/roguelike-tutorial-table-of-contents
//! [links-2]: https://engineering.intercom.io/yehuda-on-rust-with-ruby/
//!
//! # Module Docs!
//!
//! As you've no doubt noted if you've actually looked at the show notes along
//! the way, these are in fact module docs! Because we're inside a module marked
//! off by being a file, we *have* to use the `//!` style of documentation
//! comments to mark them off. However, as you'll see below, if we structure or
//! declare modules in other ways, we will not have the same restriction.
//! 
//! # Sponsors
//!
//!   - reddraggone9
//!   - [Chris Patti][sponsors-1]
//!
//! [sponsors-1]: http://podcastinit.com
//!
//! ## Become a sponsor
//!
//!   - [Patreon](https://www.patreon.com/newrustacean)
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
//!
//! # Follow
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
//!       + App.net: [@chriskrycho](https://alpha.app.net/chriskrycho)


/// This is an internal module. Note that it isn't public.
///
/// Modules may have any kind of "item" local to them. "Items" in Rust are
/// things like functions, structs, enums, traits, type definitions, and other
/// modules.
///
/// Modules have namespaces
mod internal_module {

    /// A module function, demonstrating module-public function status.
    ///
    /// This function is public to the `internal_module`, but because the module
    /// itself isn't public, neither is the function. It is available to
    /// anything which uses `internal_module`, however, as it is public at the
    /// function level.
    pub fn a_public_module_fn() {
        println!("At `internal_module::a_public_module_fn()`.");
        a_private_module_fn();

    }

    /// Another module function, demonstrating module-private function status.
    ///
    /// Since this function is private to the module, it is inaccessible to
    /// external callers (see below in `use_modules_internal()`). However, it is
    /// accessible as normal to other functions within its own module, and thus
    /// can be called by `a_public_module_fn()`.
    fn a_private_module_fn() {
        println!("At `internal_module::a_private_module_fn()`.");
    }
}


/// This is an internal module which *is* public.
///
/// External modules therefore have access to this module, not just other
/// modules within the immediate parent `e006` (file) module.
pub mod public_internal_module {

    /// A public function in a public module.
    ///
    /// Note that the name of this function is the *same* as the public function
    /// in `internal_module` above! This is one of the values of namespacing.
    pub fn a_public_module_fn() {
        println!("At `public_internal_module::a_public_module_fn()`.");
        some_private_fn();
    }


    /// A private function in a public modules.
    fn some_private_fn() {
        println!("At `public_internal_module::some_private_fn()`.");
    }
}


/// Demonstrates the use of modules and namespaces.
///
/// Modules can access other modules which are contained in the same parent
/// module as them regardless of the privacy settings. However, they cannot
/// access non-public modules which don't have the same immediate parent.
pub fn use_modules_internal() {
    println!("At `use_modules_internal()`.");

    // Calling another module's function is quite straightforward.
    internal_module::a_public_module_fn();

    // Note that we cannot access the internal module's private function. If you
    // uncomment the following line, you will see a compile error indicating
    // that the function is private.
    //
    //     internal_module::a_private_module_fn();
    //
    // Similarly, as you would expect, we have access to public functions in
    // public modules, but no access to private functions in public modules. (We
    // have already seen this, in fact, just in file-based, rather than
    // declaration-based, modules.)
    public_internal_module::a_public_module_fn();
}


// What if we wanted to use *any* public function from a given module? We
// can simply `use` that module.
//
// It is worth quoting the Rust book here:
//
// > **Note:** Unlike in many languages, use declarations in Rust do not
// > declare linkage dependency with external crates. Rather, `extern crate`
// > declarations declare linkage dependencies.
pub mod demonstrate_namespacing {
    /// We can `use` other module's contents.
    use e006::public_internal_module::*;

    /// We can also alias other modules. Note that though we don't use this
    /// until `demonstrate_aliased_calls()`, we have to put any `use` statements
    /// before any other contents of the module (so this can't go between the
    /// `demonstrate_globbed_calls()` and `demonstrate_aliased_calls()` function
    /// definitions).
    use e006::internal_module as im;

    /// Demonstrates how glob-imported `use`s works.
    pub fn demonstrate_globbed_calls() {
        println!("At `demonstrate_namespacing::demonstrate_globbed_calls()`.");
        // Having imported *everything* from `public_internal_module`, we can call
        // its public functions directly.
        a_public_module_fn();
    }

    /// Demonstrates how aliased namespaces work.
    pub fn demonstrate_aliased_calls() {
        println!("At `demonstrate_namespacing::demonstrate_aliased_calls`.");
        im::a_public_module_fn();
    }

}


/// Demonstrates that modules can be `use`d within functions.
pub fn demonstrate_use_inside_function() {
    use e006::demonstrate_namespacing as dn;

    println!("At `demonstrate_use_inside_function()`.");
    dn::demonstrate_globbed_calls();
}


/// Give an example of nested modules.
///
/// Of course, *all* the modules in this file are nested: they are part of the
/// file-level `e006` module. Here, though, we see an explicit example of that.
pub mod demonstrate_nesting {
    /// This is just a nested module.
    pub mod a_nested_module {}

    // An example of re-exporting another module's contents publicly.
    //
    // Remember, this module was initially part of a private module (but one
    // within the `e006` parent module, and therefore accessible to us.)
    // We are re-exporting its `a_nested_module` contents, with a different
    // name, as a public module. When you look at the docs, you'll see this
    // function name within the module, with the docs from the definition of the
    // function in the `internal_module` above!
    pub use e006::internal_module::a_public_module_fn as now_public_fn;
}
