//! Read the show notes, as well as fairly detailed comments on all the code
//! samples referenced in the show.
//!
//! Yes, this is a slightly crazy way of building a show notes site for a
//! podcast. See [e001: Document all the things!][e001] for more details.
//!
//! [e001]: /src/show_notes/e001.rs.html

// Set the crate-level HTML rendering rules for the documentation.
#![doc(html_logo_url = "http://newrustacean.com/podcast.png",
       html_favicon_url = "http://newrustacean.com/favicon.ico",
       html_root_url = "http://newrustacean.com/")]

pub mod e000;
pub mod e001;
pub mod e002;
pub mod e003;
pub mod e004;
