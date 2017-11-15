//! Arun Kolsheshthra
//!
//!   - **Date:** November 14, 2017 (recorded October 27, 2017)
//!   - **Audio:**
//!       + [M4A]
//!       + [MP3]
//!       + [Ogg]
//!
//! [M4A]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/interview/rbr_2017/arun_kulshreshtha.m4a
//! [MP3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/interview/rbr_2017/arun_kulshreshtha.mp3
//! [Ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/interview/rbr_2017/arun_kulshreshtha.ogg
//!
//! <audio style="width: 100%" style="width: 100%" title="Rust Belt Rust 2017: Arun Kolsheshthra" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/interview/rbr_2017/arun_kulshreshtha.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/interview/rbr_2017/arun_kulshreshtha.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/interview/rbr_2017/arun_kulshreshtha.ogg">
//! </audio>


/// Read the interview!
///
/// **C:** Hi, can you tell us your name and where you work and how you got into
/// Rust?
/// 
/// **A:** Sure. My name is Arun Kulshreshtha. I’m a software engineer at
/// Facebook, and that’s actually how I got into Rust. Um, when I joined
/// Facebook, fresh out of college, um, there was a new Rust project - a new
/// Mercurial server being written in Rust there. I joined Facebook’s source
/// control team and have been working on that server ever since the middle of
/// last year. It’s been really fun, and I’ve been learning a lot about the
/// language.
/// 
/// **C:** Sweet! Mercurial. No one knows this, probably, other than close
/// friends of mine, but I really wish Mercurial had won instead of Git. So, I
/// was really excited to see people pushing on that. Uh, how long - you said
/// about a year and a half that you’ve been writing Rust?
/// 
/// **A:** Yeah, that’s about right.
/// 
/// **C:** What has been the best or most enjoyable part of that so far?
/// 
/// **A:** Well, what I’ve really enjoyed is being able to write systems
/// programming, like, just doing systems programming, in a language that has
/// things like algebraic data types and other features from the functional
/// programming world. I myself am coming from  a mixed background where I both
/// did a lot of systems programming and a lot of functional programming in
/// languages like Haskell in college, and the functional programming stuff
/// really appealed to me, and I always wish C and C+ + had those features, so
/// it’s really great getting to use them every day in my systems work.
/// 
/// **C:** That’s awesome. So, I know that’s an open source code base - how big
/// is it now?
/// 
/// **A:** How big is the Mononoke project, which is the uh, source control
/// server? I’m not actually sure what the actual lines of code are because I
/// don’t keep up with our git hub repo from day to day, um, not everything is
/// open source unfortunately, but a good chunk of it - the majority of it - is
/// on Github. I’d say, you know, definitely several thousand lines, probably
/// tens of thousands of lines, but don’t quote me on that, because I don’t
/// actually know. But if you search on Github you’ll find the repo and you can
/// find out.
/// 
/// **C:** What, if any, have been pain points for you along the way with Rust?
/// 
/// **A:** So the biggest pain point has probably been, like, Facebook is
/// definitely a C+ + shop, as far as the back end is concerned, so all of the
/// existing libraries and APIs are all designed with C+ + in mind. One of my
/// jobs has been to write bindings to those libraries from Rust, and
/// unfortunately, while the binding situation in Rust is actually not bad, with
/// tools like bindgen to help automatically generate those bindings, it turns
/// out when you’re using really advanced C+ + features, like C+ + 14, and maybe
/// even C++ 17 features - because Facebook always has to be on the bleeding
/// edge after all - bindgen falls over, and you find yourself writing all these
/// wrappers and shims to get everything to work, and that is a bit of a pain. I
/// wish I didn’t have to do that on a near daily basis, but that is my life
/// right now. But, the pain’s worth it to get to use Rust.
/// 
/// **C:** That’s, that’s fair, I think! Uh, is there anything else - any
/// particular standouts - of your time so far here at Rust Belt Rust, or that
/// you’re looking forward to?
/// 
/// **A:** Just interacting with the community members here at Rust Belt Rust
/// has been amazing. Uh, I was also at Rust Conf for earlier this year, and I
/// found that the community is extremely approachable and welcoming. I was
/// really worried, like, I’m a new grad; I barely have any industry experience.
/// Hell, I don’t even know what I’m talking about some of the time. Like, will
/// I fit in? But, like, I can talk to, like, some of the maintainers of, like,
/// key crates, you know? They’ll have a technical conversation with me and,
/// like, give me pointers and stuff, and that’s really awesome.
/// 
/// **C:** Awesome. Thanks so much for your time, Arun.
/// 
/// **A:** Yeah, thank you!
pub struct Transcript;