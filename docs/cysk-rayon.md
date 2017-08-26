# Rayon

## Intro

Hello, my name is Chris Krycho, and this is the New Rustacean podcast – a show about learning the Rust programming language. This is Crates You Should Know: Rayon.

Before I jump into the show, two quick things to call out in the community which I would normally put in the News episode that'll be coming out next week, but which are happening before it comes out!

1. There's a Rust meetup in the Raleigh-Durham-Chapel Hill area that's now running on the fourth Monday of every month – so, August 28, September 25, etc. For at least the August and September meetings, I'll be there!
2. There's a NYC Rust meetup happening on August 31. I don't have a long-term schedule for that meetup, but you can follow up with the organizer, Robert Balicki for more details

Info about time and place for both is on meetup.com, and I'll have those in the show notes. If you'd like me to call out local meetups, I'd be happy to (usually in News episodes) – just send me a note on Twitter @newrustacean or email me at hello@newrustacean.com!

Now, into the show!

## Overview

One of the major promises of Rust is that it can do "threads without data races." For anyone who's ever done multithreaded work in most conventional languages, that sounds almost too good to be true. The only approaches which have proven capable of solving this problem are green threads, as you see most prominently these days in Go but which are also closely related to the approach used in Erlang, and trying really, really hard to be really, really careful.

As usual, that "be really, really careful" approach tends not to work out all that well in practice: as hard as you try, inevitably you end up with two different threads trying to access shared memory at the same time and things go badly. One solution to this problem in other contexts has been pure functional programming, leaning on persistent data structure implementations to make the approach ergonomic and workable. But it's difficult to apply those solutions in the high-performance, deterministic contexts many Rust users need.

As usual, Rust's approach to ownership gives us superpowers, and Rayon makes those superpowers easy to use for everyone. 

## Outro

So that's a look at Rayon! I hope you find some neat places to parallelize your code safely—this is, after all, one of the great promises of Rust!

Thanks to this month's $10-or-more sponsors:

- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Matt Rudder
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Philipp Keller
- Raph Levien
- and Vesa Khailavirta

As always, thanks as well to everyone who helps other people find the show---whether by just telling a friend, or by rating or reviewing it in iTunes or recommending it in another podcast directory, or by sharing it on social media!

You can find show notes for this episode, including links to docs and more for Rayon, at `newrustacean.com/show_notes/cysk/rayon/`. The show is on Twitter @newrustacean, and I am @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or – and this is my favorite – just send me an email at hello@newrustacean.com.

Until next time, happy coding.
