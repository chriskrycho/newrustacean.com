//! Show notes
//!
//! Read the show notes, as well as fairly detailed comments on all the code
//! samples referenced in the show.
//!
//! Yes, this is a slightly crazy way of building a show notes site for a
//! podcast. See [e001: Document all the things!][e001] for more details.
//!
//! [e001]: /src/show_notes/e001.rs.html

// Set the crate-level HTML rendering rules for the documentation.
#![doc(
    html_logo_url = "https://newrustacean.com/podcast.png",
    html_favicon_url = "https://newrustacean.com/favicon.ico",
    html_root_url = "https://newrustacean.com/"
)]
// Enable access to the benchmarking functionality. Note that with this present,
// we require using nightly Rust (as of 1.5).
#![feature(test)]
// Enable `#[doc(include = "<path>")]
#![feature(external_doc)]

// This statement gives us access to the `test` crate for benchmarking.
extern crate test;

// Make the show notes public.
pub mod e000;
pub mod e001;
pub mod e002;
pub mod e003;
pub mod e004;
pub mod e005;
pub mod e006;
pub mod e007;
pub mod e008;
pub mod e009;
// We need to include `#[macro_use]` so that the macro will be exported for
// other crates to use---or, in our case, to be visible in the docs/show notes.
#[macro_use]
pub mod e010;
pub mod e011;
pub mod e012;
pub mod e013;
pub mod e014;
pub mod e015;
pub mod e016;
pub mod e017;
pub mod e018;
pub mod e019;
pub mod e020;
pub mod e021;
pub mod e022;
pub mod e023;
pub mod e024;
pub mod e025;
pub mod e026;
pub mod e027;

pub mod bonus;
pub mod cysk;
pub mod interview;
pub mod meta;
pub mod news;
