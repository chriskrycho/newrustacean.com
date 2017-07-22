
## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast, a show about learning the Rust programming language. This is a News episode: Increasing Rust's Reach.

## Increasing Rust's Reach

### Background

One of the major goals for the entire language this year is to expand Rust's reach, and increasingly the core team has recognized (even beyond the existing commitments in this direction) that we need to make the Rust community a more diverse and inclusive place. That includes things along many lines; one big piece is going out of our way to make sure we're easy to jump in for groups of people which have often been underrepresented in the programming world. Another is even just making it so our stuff is easier to dig into whether you're under-represented or *over*-represented in the technical world in terms of things like gender, ethnicity, and so on. As awesome as Rust is, there are many people for whom the barrier to entry is still too high to just up and jump in. It is *hard* for people if this is their first language especially. But it is also hard in some ways even just to see why you might use the language – the rust-lang.org landing page need some love. Or, as Jonathan Turner pointed out in our interview, you need a lot of knowledge about Rust to be able to use the documentation of the standard library. There are *lots* of those little paper cuts.

### The Initiative

To make a real improvement in this category, the Rust team has kicked off an initiative called "Improving Rust's Reach." The goal is to lower its learning curve *by* drawing on the expertise of people who are outside the existing community, and that very well might mean *you* – whether because you're a member of those too-often-underrepresented groups, or because you're a designer or otherwise not the stereotypical picture of a low-level language hacker. The initiative is a set of small projects – driven by those same people who are outside the norm, but paired with existing Rust community leaders and empowered to tackle specific parts of the ecosystem to make it easier to get up to speed! There are there big benefits to participants:

1. You (someone who is *not* part of the main stream of current Rust users, in one way or another) get to dive in and learn a bunch about Rust from leaders in the Rust community while also making a significant contribution. That comes with, among other things, access to a private Slack team dedicated to coordinating this effort, with active feedback from .

2. You get to go to one of three Rust conferences of your choice later this year, on Mozilla's dime:

	- RustConf, August 18–19 in Portland, Oregon, in the US
	- RustFest, September 30–October 1 in Zurich, Switzerland
	- Rust Belt Rust, October 26–27 in Columbus, Ohio, in the US

3. You get some public recognition for all of your contributions!

What does working on one of these projects entail? A commitment of 3–5 hours each week between August 7 and November 6—so, a three month commitment, with the expectation that you'll have a deliverable by the end. Depending on the project and your skill set, that deliverable might vary *widely*, of course: it might be anything from a series of pull requests to new tutorials in a given are. It might be Rust code, or it might be a mix of Rust and something totally different, like React JS.

### The Projects

So, how can you get involved? Well, write a proposal for one of the projects! It’s due by July 18, so you should get on it in a hurry! You can see the full list in the announcement blog post, which I've linked in the show notes, but I thought it might be useful to include a short list of the initiatives. They are:

1. User Experience of Rust Documentation and Code Browsing Tools – Nick Cameron, core team contributor to rustdoc, rustfmt, RLS, and the compiler; and Steve Klabnik, core team docs guru and web tech enthusiast: help make it easier to use Rust's *documentation*, which will improve experience of using the standard library docs and the auto-generated docs that live at docs.rs. That needs web tech experience and an eye for design, because the docs have web front ends!

2. Adding Code Lints to the Clippy Developer Tool – Manish Goregaokar: add more and more helpful messages to Rust's main static analysis tool, including lints focused on helping people transition into Rust from other languages, tutorial-like messages, and more.

3. Improve the Approachability of the Design of rust-lang.org and/or crates.io – Aaron Turon: rust-lang.org and crates.io both need some work! The main page could use a lot more and better information on why you might want to use Rust, and crates.io could be both a lot nicer-looking and a lot easier to find your way around. If you have some design chops and some web skills, this is a place you could make a *big* difference for on boarding new users.

4. Improving the Video Tutorials at intorust.com – Niko Matsakis, one of Rust's core language developers and designers: video tutorials for Rust are still relatively rare, but they're a common way many people initially come up to speed on a new technology. You can help expand the actual video content as well as help evaluate it for what the current gaps are that might not be obvious to current Rust insiders.

5. Write Tutorials for Rocket and Diesel, Parts of Rust's Web Framework Story – Sean Griffin, creator of Diesel: we need people with experience in web development in web frameworks in the non-Rust world to help take this ecosystem from "barely bootstrapped" to "flying high." In particular, we need help putting together good teaching material—tutorials, videos, example applications, and improvements to the docs—that shows how to use Rocket, Diesel, and the two together to build real-world web applications.

6. Extending the Literate Programming Tool Tango – Felix Klock, a member of the Rust compiler and language teams: literate programming lets you write your code inline with text that describes it, so that documentation or planning and code can live side by side as a single living, working document. tango does that with Markdown and Rust, and making it more viable for the community as a whole would be a big win—and it might be particularly useful for Rust documentation: imagine if *all* Rust READMEs were in principle just "literate" Rust and you could run them!

7. Finding Missing Pieces in the Crates Ecosystem – Andrew Gallant ("burntsushi"), author of (among other things) the awesome *ripgrep*: Rust is still young, and its ecosystem of important libraries has real gaps. Andrew's idea is to build a small application in a language you're already comfortable with, and then Port it to Rust and use that experience to identify libraries present in the original language but missing in Rust. That turns into actionable knowledge for the Rust community as a whole: what crates need to be written for applications like that one?

8. Finding Missing Pieces in the Experience of Building a Command Line Interface (CLI) Program – Kamal Marhubi, maintainer of *nix*: Rust is already *decent* for writing small command line tools (as I've seen myself), but it could be *great* for it. If you're interested in building new CLIs or rewriting existing CLIs in Rust, this is the project for you. The main goal of this particular project is to identify the pain points in these kinds of projects today, and then smooth some of those rough edges out so it's easier for others to do build CLIs in the future!

If you're noticing a theme there – lots of focus on documentation, and *some* on tooling – that's for a reason! Although things like teaching materials and visual design often get treated as a second-class part of any programming ecosystem (programmers tend to want to write code!), the reality is that poor design and missing or difficult-to-use documentation can and do dramatically *raise* the bar for entry into the ecosystem. A huge part of what people need therefore is not only the ongoing. And it's also the case that, while the Rust community values especially documentation already, we have a lot of work to do to make paths into the language for people who aren't already a lot like today's Rustaceans. So again: that's where you come in! Write a proposal and get it in by July 18!

## Outro

Thanks, as always, to the show's sponsors!

- Anthony Deschamps
- Behnam Esfahbod
- Christopher Giffard
- Chris Palmer
- Dan Abrams
- Daniel Collin
- Matt Rudder
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

Thank you as well to everyone who shares the show with others, whether on social media, in the threads for the episode on Hacker News, Reddit, or the Rust user form, or by recommending in a podcast app!

Show notes for this episode are at NewRustacean.com/show_notes/news/_3/. I'm on Twitter @chriskrycho; the show is @newrustacean; and you can always send an email to hello@newrustacean.com. A number of the shows coming down the line are listener-suggested – so yours could be next on the list!

Until next time, happy coding!
