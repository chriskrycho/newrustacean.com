# e026: Functional Programming Ideas in Rust

Hello, I’m Chris Krycho, and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is episode 26, functional programming ideas in Rust.

## Sponsor: Parity Technologies

Before we jump in: Parity Technologies is back sponsoring the show again! Parity is advancing the state of the art in decentralized technology. Their flagship software is the Parity Ethereum client, but they're also building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Their next big project is Polkadot, a platform leveraging blockchain tech for scaling and interop in decentralized systems. Parity uses Rust for its trifecta of safety, speed, and correctness—and they’re hiring Rust developers! So if you’d like to work on any of these projects, check out their jobs at paritytech.io/jobs.

Thanks to Parity for sponsoring the show!

## A little context

I’ve been thinking about the relationship between Rust and functional programming essentially since the day I first *really* discovered the language, back in the summer of 2015. I was primed for Rust by reading Pat Brisbin’s little book <cite>Maybe Haskell</cite>, which uses the `Maybe` type in Haskell as a very basic introduction to functors and monads and applicatives. It blew my mind, because I had been looking for—and trying to implement in my own haphazard ways in the C and Fortran and Python codebases I was working in at the time—the ideas that `Maybe` represents. I did not come away from that little book totally grasping functors or applicatives or monads. In fact, my main takeaway was something more like: “Wait, you can get rid of `null` once and for all? You can handle it *in the type system*? YESSSSS.”

When I bumped into Rust just a few months later, the fact that `Option` and `Result` existed, along with pattern-matching, sold me 100%. I needed no further convincing.

Rust, as I’ve often put it, is basically what you would get if you mashed together C++ and Haskell. And that combination is interesting: it’s *not* exactly functional; but it’s not exactly the C++ style of imperative, object-oriented code, either. I’ve spent a lot of time talking over the last few years about how Rust is both like and unlike C++. Here, today, I thought it would be interesting to tease out some of the ways it’s similar to and dissimilar to functional programming languages.

## Functional Programming and Rust

Let’s start with some of the ways Rust is *similar* to functional programming languages – or, perhaps a more accurate way to say it, some of the ways Rust steals ideas from functional programming languages.

### Similarities

First, like most languages developed over the last few decades, Rust has a bunch of what you might call *functional-inspired idioms*. The entire `Iterator` <abbr>API</abbr>, for example, leans hard on ideas from functional programming: `map`, `fold`, and similar are probably the most common bits of cross-pollination from functional programming into imperative and object-oriented languages. You’ll find the same with C♯’s <abbr>LINQ</abbr> and Java’s Streams API, along with similar patterns in JavaScript, Python, Ruby, and so on.

Rust borrows a bit more from functional programming languages than those other languages, though, in its type system and its approach to shared functionality. Whereas most of the other languages I just mentioned put classes front and center, and use classes as their primary mechanism for shared behavior, Rust *doesn’t*. As we talked about over the past three teaching episodes, Rust separates data definitions from the functionality which operates on the data; and it puts traits front and center for shared behavior. In this, it’s stealing directly from functional programming generally with the separation between data and behavior, and from Haskell specifically with its notion of type classes as a means of shared behavior. This is, I suspect, one part of what makes Rust so *odd*-seeming to many people who come to it from languages like C++ or Java or Ruby. It’s a particular delight to me, because I (like many others) have long found the separation of data from behavior to be a useful pattern. Rust just elevates that to a prime feature of the programming language.

The last major way Rust steals from functional programming languages is in its type system. In particular, its `enum` types are bog-standard in functional programming languages. “Tagged unions” are certainly *possible* in procedural and object oriented languages—I built them or things like them myself in C, Python, and JavaScript—but they’re not first-class citizens of any mainstream imperative or OO language, and they don’t have the pattern-matching special sauce that makes them so powerful in Rust, OCaml, and so on. In fact, Rust’s `enum`s are probably my favorite of the things it borrows from FP languages, because I find them to be a particularly powerful and helpful way of expressing a lot of the kinds of problems I encounter.

In short: Rust borrows a *lot* from functional programming languages. But not everything.

### Differences

And what’s most interesting to me is that for all of those similarities, Rust*isn’t* in the end a functional programming language. It’s a language which, having stolen a ton of ideas from functional programming, turns around and adds its own spin on top of them. The major place this shows up is in Rust’s approach to mutability and imperative code—both of which it embraces, and that’s actually really important for getting the kinds of performance characteristics we get from Rust.

#### Shared mutable state

I’ve often heard it said by fans of functional programming languages that *shared mutable state is the root of all evil*. What people mean when they say this is that one of the major classes of bugs in our code is not logic errors, but simply shared state that multiple pieces of the code can write independent of each other. “Shared mutable state” makes it harder to track whether the invariants that are supposed to hold for a given piece of data *do* in fact hold.

And functional programming is far from the only programming paradigm to recognize this. There is good reason that *global* mutable state is widely recognized to be a disastrously bad idea in *all* modern programming languages. And in fact, much of what we do in an object-oriented style is a matter of managing the scope of mutable state. We encapsulate it into objects and restrict access to it – when there are specific constraints that have to be upheld for a given data structure, we’ll often prevent *any* direct access to it in favor of methods which do perform a mutation, but which we can guarantee perform that mutation *safely*. For example, you can think of a class which wraps a custom queue implementation, where the queue is just implemented as a array with indices that track the first and last positions in it. The vector is mutable state, but we minimize how we *share* it: we don’t expose it for others to access, and we control how you can add items to the end and pop them off the front.

Functional programming’s insight is that even this model often ends up causing us problems, not least because as a system grows it can be difficult to understand (and therefore to continue enforcing) those invariants. More, the very statefulness of those data structures can often lead us into places where we have *implicit ordering* – all the steps you have to do to get the data structure into a certain shape, usually in the form of a bunch of method calls which return `void` and, in good encapsulation form, give me no idea how the data structure underneath has changed. I know I’ve written plenty of code that involved an absurd amount of setup to test, and that’s a pretty big code smell.

Functional programming usually solves this problem by dropping the mutability from the equation. It’s not a problem to have shared state if it isn’t mutable; everyone just has a read-only copy of the data and you create new copies of the data when you need it. Functions just take in data and hand back new data, without altering the original, even when they’re a transformation of the original. That probably sounds *super* expensive performance-wise, but we have smart tools for making that cost a lot less than it seems like it would; I’ll link to notes on persistent data structures if you’re unfamiliar with that idea! I’ll also link to a couple persistent data structure libraries in Rust, because this idea is useful here, too.

I find this approach very powerful. It lets me *know* that if I have a given piece of data, it won’t be changed underneath me (no matter what function I pass it to!). It means testing is much simpler: just *create* the data I want to pass in directly, rather than making sure I do the right sequence of method invocations to hopefully get that shape right. All these things are great!

They’re also expensive, though, performance-wise. Even using persistent data structures doesn’t get rid of that totally. Often, for application-level code, it doesn’t matter that much. But in some domains, at some times, it *does* matter. At the end of the day, mutation is *actually what happens* at the level of memory in a lot of cases, and abstracting over that has a cost, plain and simple. As such domains where pure functional programming isn’t 100% viable... tend to have an awful lot of overlap with Rust.

#### Rust’s pitch: allow both, but never together

Rust doesn’t give up on this insight about shared mutable state, though. It just takes a step back and says, “What if we allowed *both* shared state *and* mutable state but *never at the same time* and always make the relationship explicit?” In other words, it’s the same problem, but a slightly different solution.

Rust, like functional programming languages, will *not* let us mutate shared state, because, well, all of the reasons we said above! But it *will* let us mutate *state that isn’t shared*. The entire point of Rust’s ownership rules and the borrow checker is to enable this additional way of solving the shared-mutable-state problem. I said a minute ago that it’s not a problem to have shared state if it isn’t mutable. But the flipside is also true: it’s not a problem to have mutable state if it isn’t shared.

This might at first sound like we’re just back to encapsulation land, and that’s not *totally* wrong. Rust *does* let us keep internal data structures private, and we *do* lose some of those benefits of the total transparency from purely functional programming. However, the ownership rules go all the way down, you might say – and all the way up. In most higher-level languages, multiple objects can have references to a given data structure at the same time, and as such they can be calling methods on that data structure independently of each other, resulting in independent mutation of the underlying state. And because of the kinds of indirection in play, this can easily enough happen right under your nose: you call a method on one object, not realizing it updates another data structure “under the covers,” and then call another method on a different object, not realizing *it* does the same thing, and then go to look at the data and find yourself scratching your head at how the same value ended up in the target data structure twice.

None of that is true in Rust! If a method is going to mutate inner state on the data structure it’s implemented for, it’s going to have to take self by mutable reference, and that means that whichever data structure or function is dealing with it has to have a mutable reference to it as well... and per Rust’s ownership rules, that has to be a *unique* mutable reference. We simply don’t have the problem where multiple different owners acted on a piece of data independently of each other, because no piece of data in Rust can have multiple owners.

#### Ownership example

To make this a little more concrete, let’s go back to our custom queue example from a few minutes ago. We might use a `VecDeque` under the hood, but intentionally not expose that in our `Queue` type publicly. In this case, the `Queue` would own the `VecDeque`. Its implementation would likely have `push` and `pop` methods, both of which would *have* to take self by mutable reference.

That means that if you have two *other* data structures in your system which would like access to the queue, they cannot be in one of those states where one of them still has this mutable access to your `Queue` while the other doesn’t, and *this* means that you can’t have that weird thing where a side-effecting operation on one item in your system sneakily updated a data structure “behind your back.” The type system and ownership rules make it explicit who’s allowed to touch the `Queue`. You’d see right away – or at least, soon enough, when the compiler notified you! – that the intermediate data structures both tried to change the underlying data.

And, perhaps most importantly, you’d generally have to do that by explicitly passing around the `Queue`, because you can *never* have two other data types which themselves both had mutable references to another piece of data at the same time, and you couldn’t even have two other data types where one of them had a reference *at all* while the other had a mutable reference.

This doesn’t prevent you from making *logic* errors in this space, of course. You could intentionally get your lifetimes all in a row to make it so you could doubly-insert that data. But at a minimum, it would be much more obvious that was what you were doing! We’ve expanded the set of solutions to the “shared mutable state is the root of all evil” problem: we can write in a pure functional style (and that’s often quite useful in Rust!), but we can also take advantage of controlled mutability—emphasis on *controlled*!

## Closing

I hope you found all of that helpful if you’re wondering about the relationship between Rust and functional programming! Certainly the way the two interact has been interesting to me over the past few years!

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- Behnam Esfahbod
- David W. Allen
- Derek Buckley
- Matt Rudder
- Chris Palmer
- Vesa Khailavirta
- Nick Stevens
- Aaron Turon
- Dan Abrams
- Anthony Deschamps
- Chip
- Ryan Osial
- Nathan Sculli
- Daniel Mason
- Martin Heuschober
- Raph Levien
- Peter Tillemans
- Marshall Clyburn
- Sascha Grunert
- Alexander Payne
- Paul Naranja
- Rob Tsuk
- Zachary Snyder
- Daniel Collin
- Hans Fjällemark
- Oluseyi Sonaiya
- Ramon Buckland
- John Rudnick
- Damien Stanton

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for \_this\_ episode at \<newrustacean.com/show\_notes/bonus/functional\_programming\>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
