# Bonus: I Still Make Silly Mistakes

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a bonus episode: I Still Make Silly Mistakes.

## Sponsor: Parity

First up, Parity is back sponsoring the show again because they want *you* to come write Rust with them! Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## That mistake…

So, the mistake I titled this episode after. Every time I draft an episode of New Rustacean, I start by writing out a script for it, long-form, in my preferred Markdown editor. (You can find those scripts in the show notes!) When I record the show, I’m reading that script—usually with some minor improvisation. One of the things I do as I’m writing is include links inline in the script. I do that for two reasons:

- first, so that anyone who is reading the script has an easy way to link to the things I’m talking about throughout the episode
- and second, so that when I pull that script over into the repository for the show, I can just pull those links out and put them in the show notes

The first part works fine. Pulling the links out for the show notes has increasingly annoyed me, though: I have to go through and manually pull out each link from the text, and while that’s not a *ton* of work, it’s repetitive and error-prone. And “repetitive and error-prone” is basically the working definition of what software is good at! So I figured: hey, here’s a chance to play with a couple data structures I’ve not spent much time with in Rust and get more familiar with the [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) crate, which has a nice API for dealing with Markdown!

(As I mentioned in Part I of the Rust 1.31 and 2018 Edition news episode, I basically default to Rust for these tasks at this point: it takes me about the same amount of time as doing in a scripting language would, and I enjoy it a lot more.)

So I did what I normally do in these kinds of situations and ran `cargo new extract-md-links`. I pulled in the `pulldown-cmark` in my `cargo.toml`, and debated whether I wanted to pull in [the `structopt` crate](https://github.com/TeXitoi/structopt) for command-line argument handling—and decided against it because I figured: all I need to be able to do is pass in a single argument and this is a private tool for my very specific niche problem. No problem; I’ve parsed arguments from the command line manually for over a decade!

Right?

Wrong!

I whipped up my main function and started trying to load a test document to start on the link extraction, figuring that would be the meat of this particular little problem. And then I started seeing an error that made *no* sense to me: `"stream did not contain valid UTF-8"`.

What.

Look: I *know* this file is valid UTF-8. It’s a blog post that currently runs through a Python-powered static site generator; it wouldn’t even have successfully published if it were *not* valid UTF-8. What's more: the file opens as valid UTF-8 in Visual Studio Code and Sublime Text. It validates properly as UTF-8 when I dump it into an online UTF-8 validator. It loads properly if I open a Python REPL and load the exact same path I was passing to my little Rust tool!

At this point, as you can probably imagine, I started to feel like I was losing my mind. I hopped into the Rust Discord channel and started asking *really* basic questions about the things I was seeing. As an aside, if you've never just hopped into one of the Rust chat communities online, you should do that the next time you get stuck. And the rest of this story is a great example of *why*, but the short version is that the Rust community is *incredibly* helpful. Even for silly problems where the question-asker really ought to know better! But we'll get to that.

So I started describing my problem, and a few other Rust users—especially @alercah—started asking helpful questions and offering helpful comments about the file I was trying to work with:

- **What’s the first byte?** So instead of running `std::fs::read_to_string`, I used `std::fs::read`, which gives you back a `Vec<u8>` if it succeeds. It succeeded, so dumped the first byte.
- **That doesn’t seem right; what’s the *second* byte?** At this point I just grabbed the first *eight* bytes the same way and shared those.
- **Those are definitely not valid unicode. What’s the file?** I linked directly to the file I was trying to load from my blog source on GitHub.
- **That file starts with different bytes than the bytes your Rust program is reporting.** And indeed, it started with the bytes I’d have expected: three hyphens (which are what start a block of YAML metadata for my blog posts).

That last point had me *truly* confused. How in the world could two different people read the same file and get different results? And this of course was the key to the whole thing. But I'm getting ahead of myself again.

My next step was to just dump the minimal set of code to reproduce this program straight into a code block in Discord, along with the actual Cargo command I was running and the actual error I was seeing. And then @alercah spotted my problem, and it turned out it had nothing at all to do with the file I thought, or with unicode at all. My problem was simply that I was *opening the wrong file*.

You see, the first argument to a program is always the name of the program itself. There’s good reason for this; I’ve linked [an interesting discussion][unix] in the show notes (using this tool, which now works!) about why you might want the same program to be able to be called with different names and do different things in those circumstances. That's all just fine and dandy. Except that I was thinking the first argument would be the thing I was passing to the program: the path to the file I wanted to process. Instead, it was, you know: not unicode. Because it was a binary: the Rust program I was running! Once I grabbed the *second* argument, not the *first*, everything just worked exactly the way I expected.

[unix]: https://unix.stackexchange.com/questions/315812/why-does-argv-include-the-program-name "Why does argv include the program name?"

So that's what happened *technically*, that's the bug. But, and this is the actually interesting part of this experience, I think: you may recall my comment a minute ago that I’ve been doing this—parsing command line arguments—for over a decade! And I mean that literally: I first parsed command line arguments manually in Fortran and Java programs in 2008. This is a quote-unquote “rookie mistake.” And I laughed *really* hard at myself when I realized what had happened because it's such a "rookie mistake"!

But of course, that phrase is really misleading. “Rookie mistake” suggests that these kinds of mistakes are specific to rookies, and the reality is that they *aren’t*. I'm not a rookie a decade in, at least not at this particular thing. But these kinds of things happen to all of us. And "these kinds of things" are just mistakes that come from *unfamiliarity*. Being new to programming entirely is one reason you might be unfamiliar with something. But just being rusty—no pun intended!—is another reason you might be unfamiliar with something! I haven’t had to hand-parse an argument in about five or six years, because I’ve been using something like [Python’s `argparse` library][argparse], or [Commander in Node][commander], or [clap] and [structopt] in Rust! And that meant it was easy to forget this kind of thing because I just hadn’t had to do it manually for such a long time. So that was the first reminder: *everyone* makes "rookie mistakes," because they're not really *rookie* mistakes; they're unfamiliarity mistakes.

[argparse]: https://docs.python.org/3.7/library/argparse.html#module-argparse
[commander]: https://github.com/tj/commander.js
[clap]: https://clap.rs
[structopt]: https://github.com/TeXitoi/structopt

The second good reminder for me in this was actually from the feeling of frustration itself. It reminded me of what it’s like to be just starting out—to be a rookie!—to be looking at a compiler message that seems like it’s trying really hard to be helpful, but which is clearly telling you something that isn’t *exactly* the problem: it's not the *root* of the problem. “Yes, it’s true," I was saying, "that this is not unicode. But I have no idea *why* it isn’t unicode!” And even when you’re a decade along in your career and have generally developed a reasonable set of debugging techniques and intuitions about why things go wrong, things like this can be totally mystifying—still! And this should remind us—it reminded me—to be sympathetic to people just getting started—because even things which are totally obvious to us can be genuinely confusing to newcomers, *or even to us ourselves* with just a little distance from the specific expertise we’ve developed!

## Outro

Anyway, that’s my fun story and my two takeaways about a silly mistake I made recently; I hope it’s helpful and encouraging when you make your own silly mistakes or when you see someone make what seems like a silly mistake!

Thanks as always to this month’s $10-or-more sponsors:

- Soren Bramer Schmidt
- Graham Wihlidal
- Benjamin Manns
- Matt Rudder
- Bryan Stitt
- Brian McCallister
- Evan Stoll
- Oluseyi Sonaiya
- Nathan Sculli
- Ryan Osial
- Martin Heuschober
- Andrew Dirksen
- Daniel Collin
- Daniel Mason
- Dan Abrams
- Nick Gideo
- Behnam Esfahbod
- Scott Moeller
- Adam Green
- Nicolas Pochet
- Alexander Payne
- Nick Stevens
- Peter Tillemans
- Michael Mc Donnell
- James Hagans II
- Joseph Schrag
- Raph Levien
- Chris Palmer
- Anthony Deschamps
- David Carroll
- Jerome Froelich
- John Rudnick
- Jason Bowen
- Ramon Buckland
- Embark Studios
- Johan Andersson
- Jonathan Knapp
- Rob Tsuk
- Jako Danar
- Paul Naranja
- Chip

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/bonus/_13/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!