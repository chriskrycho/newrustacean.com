# Bonus: Translating Between Languages

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is a bonus episode on translating between programming languages.

## Sponsor: [Manning Books/Rust in Motion](http://bit.ly/2OXnlEb)

First up, a new sponsorship! I’m *really* excited to be partnering with Manning Books to bring you a discount on some *video content* they’ve put together. Even better because that video content is by Carol Nichols (whom I interviewed a couple years ago) and her husband Jake Goulding! I’ve been saying for years that we need more video content: it’s a huge way people learn. Manning’s Rust in Motion is one of the first major video courses for Rust I know of. You can get it for 40% off at [deals.manning.com/new-rustacean](http://bit.ly/2OXnlEb) – there’s a link in the show notes. That link actually gives you 40% off of *anything* at Manning, including their book *Rust in Action*, which is in early access preview. Thanks so much to Manning for sponsoring the show and building some great video content with Carol and Jake!

## Translating from TypeScript

I’ve mentioned a number of times on the show that my day job is building a web front-end in JavaScript and TypeScript—these days, pretty much entirely TypeScript. I recently had an opportunity come up to work on a side project where I am converting some library code from TypeScript to Rust, to accelerate this project via WebAssembly in the browser. I hope to have more to say about that project in the future; for now, what’s most interesting to me for this little bonus episode is what that *experience* has been like. There’s an interesting nugget here for reflecting on programming languages and what makes them valuable.

I’ve said to a number of people over the last year that writing TypeScript and writing Rust can feel very similar. Their surface syntax has a lot in common, and depending on how you write your TypeScript, the semantics can end up feeling relatively similar as well. “Depending on how you write your TypeScript,” is an important qualification. There are a *lot* of ways to write TypeScript, because there are a lot of ways to write JavaScript, and TypeScript is just type annotations on top of JavaScript. If you’re writing TypeScript like it’s C# or Java, it won’t feel much like Rust at all. But if, like me, you prefer a more functional idiom and you use TypeScript kind of like a weird, extremely verbose version OCaml, with carefully controlled mutation, well, it ends up feeling a lot like Rust. After all: Rust is kind of like OCaml, but with explicit control over ownership and mutation! So in my experience, writing TypeScript and Rust felt pretty similar, in very good ways, a lot of the time.

But. Let me tell you. When you start trying to translate things from TypeScript to Rust, stuff gets really weird, really fast.

One of the things that initially feels very similar between the two, for example, is the type system. And so far as it goes, that feeling is not totally wrong. Both languages give you generic types; both give you interface-like ways of expressing constraints and let you bound generics with those interface-like structures—literally `interface` in TypeScript and of course `trait` in Rust.

There are important differences in both the surface syntax and the underlying semantics, especially in the languages’ type systems, though, of course. The most obvious difference at the *surface* is that TypeScript has a structural type system, where it only cares about the *shape* of the objects you pass, where Rust has a nominal type system: two structs with the same fields are still different structs, because they have different *names*. What’s more, TypeScript has anonymous, bare union types but doesn’t have built-in tagged unions like Rust’s enum types. So in TypeScript you can say something is a string or a number, without any wrapping name; but you have to write a ton of boilerplate to get the equivalent of Rust’s `enum` types. And vice versa: in Rust you get *tagged* unions for free, but you can never have a *bare* union type: you always have to do the boilerplate of wrapping those kinds of things in an enum. So there are real differences at this level!

But while I thought those would be the thing that would end up leading to the biggest mismatch, it turns out that accommodating for *those* isn’t that hard. You have to write some enum types out more explicitly, and you can throw away some of the workarounds and boilerplate that come from not having Rust’s `enum` tagged union types in TypeScript, but it’s not that bad, because the semantics are relatively similar, even with those differences in surface syntax.

No, what gets really weird is the combination of TypeScript’s `interfaces` *not* being isometric with Rust’s `traits`, and *ownership*. In retrospect, the latter should have been obvious, but the `interface`/`trait` distinction was a lot *less* obvious.

Let’s start with the obvious one, and then we’ll use it as a way of circling back around to the `interface`/`trait` thing. TypeScript’s approach to interfaces and classes is (mostly) like interfaces and classes in most object oriented languages you’d be familiar with: both interfaces and classes can include both data and functions. Rust… doesn’t do that. At all. As we’ve talked about in recent teaching episodes, Rust makes a clean separation between *data* and *behavior*. `struct` and `enum` types have data. `trait` types have behavior. And never the twain shall meet.

Layered on top of that is the ownership system: TypeScript is perfectly happy to let you have lots of shared mutable references to the same data, recursive types are be fairly idiomatic, and cycles between data structures are no big deal (though they *can* cause memory leaks if you’re not careful). All of this is perfectly normal for a garbage-collected language! Meanwhile, Rust has as its most basic distinctive the *every piece of data has a single owner* rule. Recursive types are *not* all that idiomatic, and have to be explicitly wrapped in some kind of pointer type; and shared mutable references to the same data are right out. Cyclical data relationships in Rust are just a quick way to pain: you can make them work, sometimes, but it’s hard and never natural.

So if you go to try to translate a TypeScript `interface` straight into a Rust `trait`, or a TypeScript `class` straight into a Rust `struct`, *things get weird.* This is because of the real differences between the two languages is in their base semantics—the deep structure and most important constraints of the languages. These aren’t things that are apparent from surface syntax, since the surface syntax of the two is pretty similar!

Now, as an aside, this doesn’t mean syntax doesn’t matter. People often step from “syntax doesn’t map perfectly to semantics” to “syntax is irrelevant,” but the second doesn’t follow from the first. Syntax can map more or less cleanly to the underlying semantics, and it can lead—or mislead—by the way it’s shaped. So what we ought to say is that while syntax matters, it’s far from the *only* thing that matters.

What I have ended up finding as I’ve worked on this translation project is that sometimes an `interface` in TypeScript translates to a `trait` in Rust… but sometimes it should be a `struct` or an `enum`! It depends on how it’s used and what the mechanics in play are. Likewise, things like an abstract base class in TypeScript don’t have *any* corresponding notion in Rust, so you have to dig a bit on what the intent is and then spend a while thinking about how you might express that intent in Rust.

More, sometimes it may not be possible to express exactly the same thing in Rust in any idiomatic way, or sometimes even at all. For example, as I noted a moment ago, TypeScript supports cyclical types: it’s just modeling the way JavaScript behaves at runtime, and JavaScript types can and do have cycles. If you want an object with a reference to its parent which references the item as its own child, well… Rust doesn’t have a fun time with bidirectionally linked tree structures. You can *make* it work with `Rc` (or `Arc`) and `Weak`, but if you then need to handle, say, partial equality with the `PartialEq` trait, you need to define local newtypes to avoid the orphan rule, and so on.

The net of all of this is just an interesting little observation on the realities of how different languages lead us to solve problems in different ways. Things that make very good sense in TypeScript are so difficult or un-idiomatic in Rust that you would just *never* do it that way. And the inverse is true as well, of course! There are plenty of things that Rust’s type system makes easy that are much harder to do in TypeScript, since Rust can (and does) assume that its types are useful inputs for defining runtime behavior and TypeScript has to just erase all of its types entirely, with JavaScript’s runtime behavior only *hopefully* matching the types!

And that is a good reminder of why I think being a polyglot programmer is *really good*. Different languages, even the ones which are superficially similar, help us *think* in different ways.

## Closing

I’m sure I’ll keep learning as I keep pushing forward on this, but I thought it would make for an interesting little aside here—thanks for listening, and I hope you enjoyed it!

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- Behnam Esfahbod
- Ryan Osial
- John Rudnick
- James Hagans II
- Peter Tillemans
- Alexander Payne
- Michael Mc Donnell
- Chip
- Dan Abrams
- Joseph Marhee
- Bryan Stitt
- Rob Tsuk
- Daniel Mason
- Oluseyi Sonaiya
- Adam Green
- Nathan Sculli
- Graham Wihlidal
- Scott Moeller
- Joar Wandborg
- Nick Stevens
- Raph Levien
- Daniel Collin
- Chris Palmer
- Anthony Deschamps
- Nicolas Pochet
- Messense Lv
- Bryce Johnston
- Paul Naranja
- Ramon Buckland
- Matt Rudder
- Martin Heuschober
- Jonathan Knapp
- Aaron Turon

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, send a one-off at any of a number of other services listed at newrustacean.com, or get in touch directly. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for \_this\_ episode at \<newrustacean.com/show\_notes/bonus/functional\_programming\>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
