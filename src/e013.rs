/*!
 Staying alive

 - **Date:** April 4, 2016
 - **Subject:** Reasoning about and using lifetimes in Rust (and why we need them)
 - [**Audio**][mp3]

 [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e013.mp3

 <audio style="width: 100%" title="e013: Staying alive" controls preload=metadata>
   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e013.mp3">
 </audio>


 Notes
 -----

 Lifetimes are our way of reasoning about how long a given piece of data is
 available and safe to use in Rust. The reason we don't have the dangling
 pointer problem is that we *do* have lifetimes instead. They're not magic,
 they're just a bit of semantics and syntax that let us specify the rules for
 how long any given item lives, and how long references to data must be valid.


 Links
 -----

   - [Diesel][l1]
       + [Tutorial][l2]
       + [API docs][l3]
   - [Mio, an I/O library][l4]
       + [Getting Started][l5]
       + [API Docs][l6]

 [l1]: http://diesel.rs
 [l2]: http://diesel.rs/guides/getting-started/
 [l3]: http://docs.diesel.rs/diesel/index.html
 [l4]: https://github.com/carllerche/mio
 [l5]: https://github.com/carllerche/mio/blob/getting-started/doc/getting-started.md
 [l6]: http://rustdoc.s3-website-us-east-1.amazonaws.com/mio/master/mio/


 Sponsors
 --------

   - Aleksey Pirogov
   - [Chris Palmer]
   - [Derek Morr]
   - Hamza Sheikh
   - Lachlan Collins
   - Leif Arne Storset
   - Luca Schmid
   - Micael Bergeron
   - [Pascal Hertleif]
   - Ralph Giles ("rillian")
   - Ralph "FriarTech" Loizzo
   - reddraggone9
   - Ryan Ollos
   - Vesa Kaihlavirta
   - [William Roe]

 [Chris Palmer]: http://red-oxide.org/
 [Derek Morr]: https://twitter.com/derekmorr
 [Pascal Hertleif]: https://pascalhertleif.de/
 [William Roe]: http://willroe.me

 (Thanks to the couple people donating who opted out of the reward tier, as
 well. You know who you are!)

 ### Become a sponsor

   - [Patreon](https://www.patreon.com/newrustacean)
   - [Venmo](https://venmo.com/chriskrycho)
   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
   - [Cash.me](https://cash.me/$chriskrycho)


 Contact
 -------

   - New Rustacean:
       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
   - Chris Krycho
       + GitHub: [chriskrycho](https://github.com/chriskrycho)
       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)


 Examples
 --------

 Here's an example of a function which won't actually compile. The reason is:
 the item we're trying to return a reference to (`cast`) is created inside the
 block and therefore goes out of scope at the end of the block. Putting a
 lifetime declaration on it is irrelevant!

 ```rust,ignore
 fn bad_ref_in_ref_out<'a>(num_ref: &'a i64) -> &'a f64 {
     // We can create a local binding using the input.
     let cast = *num_ref as f64;
     // We can even create a reference to it.
     let cast_ref: &'a f64 = &cast;
     // What we can't do is return the reference, because `cast` itself will go
     // out of scope on the next line and be cleaned up.
     cast_ref
 }
 ```
*/

/**
 An individual person defined in a way that includes a *reference* type.

 The lifetime of the reference member, `name`, needs to match the lifetime of
 the `Person` itself. We'd end up with a pretty weird scenario otherwise!

 Note that the lifetimes of the members match the lifetime declared for the
 type itself, so the references must live *at least as long* as the instance of
 the type to which they're attached.
*/
pub struct Individual<'a> {
    /// The person's name, as a string *reference*.
    pub name: &'a str,
    /// Just in case we're talking about a character from Genesis
    pub age: i16,
}

// TODO: over the next week, fill this out!
/// Note that the implementation block *also* has the lifetimes declared.
impl<'a> Individual<'a> {}

/**
 A reference to a number, either integral or floating-point. Goofy, yes.

 The same basic rules apply to enumerated types as to struct types. If you
 don't include the lifetime on the type itself, the compiler won't let it pass
 when you try to use the enum.

 As in the `struct` example, the lifetimes of both items match the lifetime
 declared on the type itself.
*/
pub enum NumericReference<'a> {
    IntRef(&'a i64),
    FloatRef(&'a f64),
}

// TODO: over the next week, fill this out!
impl<'a> NumericReference<'a> {}

/**
 Get a(n optional) sub-slice of a slice.

 Note that here we have a reference coming both directions, and therefore
 lifetimes on both the input and the output.

 What we're doing here is returning a *reference* to everything but the "head"
 of the slice. Since the slice might be empty, or might have only one element,
 we treat this as an optional.
*/
pub fn refs_all_around<'a>(input: &'a [i32]) -> Option<&'a [i32]> {
    if input.len() > 2 {
        Some(&input[1..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refs_all_around() {
        let the_thing = [1, 2, 3, 4];
        let result = refs_all_around(&the_thing);
        assert_eq!(the_thing[1..], *result.unwrap());
    }
}
