//! Read the show notes, as well as fairly detailed comments on all the code
//! samples referenced in the show.
//!
//! Yes, this is a slightly crazy way of building a show notes site for a
//! podcast. But more on that in the future.

#![doc(html_logo_url = "http://newrustacean.com/podcast.png",
       html_favicon_url = "http://newrustacean.com/favicon.ico",
       html_root_url = "http://newrustacean.com/")]

pub mod e00;
pub mod e01;

#[test]
fn it_works() {
    println!("It really does work! But this isn't much of a unit test...");
}
