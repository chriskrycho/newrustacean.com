//! `Borrow`, `AsRef`, `Deref`: my head hurts now
//!
//!   - **Date:** January 31, 2017
//!   - **Subject:** Three traits which are essential for designing good, Rustic APIs.
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e018.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e018.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e018.ogg)
//!
//! <audio title="`Borrow`, `AsRef`, `Deref`: my head hurts now" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e018.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e018.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e018.ogg">
//! </audio>
//! 
//! 
//! Notes
//! -----
//! 
//! `Borrow`, `AsRef`, and `Deref` are a little complicated, but they're
//! well-worth understanding. Together, they give you tools for dealing with
//! everything from `HashMap` and friends to conversions involving smart pointer
//! types to easily using `String` and `str` or `Vec` and slice together.
//! 
//! 
//! Links
//! -----
//!
//!   - `AsRef`, `Borrow`, and `Deref`:
//!       + [`Borrow` and `AsRef`][book-borrow-asref] in _The Rust Programming Language_
//!       + [`collections::borrow::Borrow`][borrow]
//!       + [`std::convert::AsRef`][asref]
//!       + [`std::ops::Deref`][deref]
//!   - [persistent data structures][pds]
//!   - ["Rust and Rest"] – Arnin Roacher
//!   - [sentry-cli]
//!   - Particularly relevant previous episodes:
//!       + [e008: Just like something else][e008]
//!       + [e009: Composing a Rustic tune][e009]
//!       + [e017: Point me where I need to go][e017]
//!       + [interview::2: Raph Levien][interview 2]
//!
//! [book-borrow-asref]: https://doc.rust-lang.org/stable/book/borrow-and-asref.html
//! [borrow]: https://doc.rust-lang.org/stable/collections/borrow/trait.Borrow.html
//! [asref]: https://doc.rust-lang.org/stable/std/convert/trait.AsRef.html
//! [deref]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
//! [pds]: https://en.wikipedia.org/wiki/Persistent_data_structure
//! ["Rust and Rest"]: http://lucumr.pocoo.org/2016/7/10/rust-rest/
//! [sentry-cli]: https://github.com/getsentry/sentry-cli/
//! [e008]: http://www.newrustacean.com/show_notes/e008/
//! [e009]: http://www.newrustacean.com/show_notes/e009/
//! [e017]: http://www.newrustacean.com/show_notes/e017/
//! [interview 2]: http://www.newrustacean.com/show_notes/interview/_2/index.html




use std::borrow::Borrow;
use std::convert::AsRef;
use std::ops::Deref;


/// A struct for showing that you cannot use `Borrow`, `AsRef`
pub struct NoImplsAtAll {
    contents: [u8; 8],
}

impl NoImplsAtAll {
    pub fn new(contents: &[u8; 8]) -> NoImplsAtAll {
        NoImplsAtAll { contents: contents.clone() }
    }
}


/// Demonstrate borrowing the internal contents of the item.
///
/// Note that, because of the specifics of what we're doing here, the
/// implementations of `Borrow`, `AsRef`, and `Deref` here are identical. This
/// will often, but not always, be the case, depending on the types and
/// conversions in question.
pub struct HasAllTheImpls {
    contents: [u8; 8],
}


impl HasAllTheImpls {
    pub fn new(contents: &[u8; 8]) -> HasAllTheImpls {
        HasAllTheImpls { contents: contents.clone() }
    }
}


impl Borrow<[u8]> for HasAllTheImpls {
    fn borrow(&self) -> &[u8] {
        &self.contents
    }
}


impl AsRef<[u8]> for HasAllTheImpls {
    fn as_ref(&self) -> &[u8] {
        &self.contents
    }
}


impl Deref for HasAllTheImpls {
    type Target = [u8];
    
    fn deref(&self) -> &[u8] {
        &self.contents
    }
}


/// Take it implementing `Borrow<[u8]>`.
pub fn takes_a_borrowable<B: Borrow<[u8]>>(b: B) -> Result<(), ()> {
    for el in b.borrow() {
        println!("el is {}", el);
    }

    Ok(())
}


/// Take it implementing `AsRef<[u8]>`. Note similarity to `takes_a_borrowable`.
pub fn takes_a_reference<A: AsRef<[u8]>>(a: A) -> Result<(), ()> {
    for el in a.as_ref() {
        println!("look ma, a reference! {}", el);
    }
    
    Ok(())
}


/// Take the same type by `Deref` coercion at the call site.
pub fn coerces_via_deref(coerced: &[u8]) -> Result<(), ()> {
    for el in coerced {
        println!("we borrowed it as a straight-up reference: {}", el);
    }
    
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e018_demonstrate_borrow() {
        let to_borrow: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(Ok(()), takes_a_borrowable(to_borrow));
        assert_eq!(Ok(()), takes_a_borrowable(to_borrow.to_vec()));

        let mut borrow_this_too: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(Ok(()), takes_a_borrowable(borrow_this_too));

        let _contents_not_borrowable = NoImplsAtAll::new(&to_borrow);

        // would fail:
        // takes_a_borrowable(&_contents_not_borrowable);

        let contents_borrowable = HasAllTheImpls::new(&to_borrow);
        assert_eq!(Ok(()), takes_a_borrowable(contents_borrowable));
    }

    #[test]
    fn e018_demonstrate_as_ref() {

    }

    #[test]
    fn e018_demonstrate_deref() {
        let basic: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(Ok(()), coerces_via_deref(&basic));
        
        let to_coerce = HasAllTheImpls::new(&basic);
        assert_eq!(Ok(()), coerces_via_deref(&to_coerce));
    }
}
