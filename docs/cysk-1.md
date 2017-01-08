# `quick-xml`

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Crates You Should Know: `quick-xml`*.

## The crate

As part of my work on my Lightning static site generator, I realized I needed to be able to quickly parse XML for syntax highlighting: I want to be able to do that at build time, rather than running JavaScript on the page when a user sees it. (I'll talk about the crate I'm using to do *that* in a future crates-you-should-know episode!)

The challenge I faced was taking a bunch of well-formed HTML and reliably identifying every language-tagged code block, extracting it, syntax-highlighting it, and then replacing it in the original content. As you know if you've ever tried to do this kind of thing in HTML or XML, you *might* be able to get away with handling this with a regular expression *most* of the time, but eventually that approach will blow up in your face. This is a job for an XML parser and writer! (HTML is roughly a subset of XML. It's not strictly a subset, but usually you'll find that XML parsers can also handle HTML.)

So I went looking for an XML crate! There are a number of XML-processing crates in the Rust ecosystem presently, and I read through the docs for each of them and played with a couple of them. I landed on `quick-xml`: it's fast, it has a fairly ergonomic API, and it supports XML namespaces. (If you're wondering, I don't need namespaces for *this* project, but lightning is not just about static site generation for me; it's also a proving ground for a bunch of *other* ideas I'm working on.)

So let's talk about that API a little! The crate is pretty straightforward. At the top level, it exposes four structs and one enum: `Element`, `XmlDecl`, `XmlReader`, and `XmlWriter`. There are also some types for handling attributes, errors, and namespace management, off in their own dedicated modules. The `Element` type is the workhorse of the struct types, and the reader and writer types and methods use it heavily in conjunction with the `Event` enum. The `Event`s are things like start tags, end tags, comments, text, and so on---each variant contains an `Element`. `XmlDecl` is just a wrapper around an `Element` with some extra functions for handling the specific attributes you care about from XML declarations specifically. The `XmlReader` and `XmlWriter` types do exactly what they say: give you an interface for reading and writing XML.

Now, `quick-xml` is implemented as a "pull parser," meaning that instead of preemptively handing you a fully parsed tree or something along those lines, it works as an iterator, which in turn allows you to step through it as you will. There are upsides and downsides to this approach. The biggest downside is that if you want a tree structure, you have to build it yourself. The biggest upside is that if you *don't* need a tree structure, you don't pay the cost of having it anyway. That's particular interesting for memory usage reasons. I'm not *overly* worried about that in the case of Lightning---it should be a memory hog---but I also take the view that if you *can* do things in a more lightweight fashion, it's often good to do so. That's a big part of what attracts me to Rust, in fact: the ability to do things that are fast and lightweight on a user's machine. In any case, the net result of making `quick-xml` be a pull parser is that parsing and writing some XML with the crate was pretty easy.

Here's how it works: The `XmlReader` takes a buffer of content and produces a stream of `Event`s, and the `XmlWriter` takes `Event`s and writes them to anything which implements the `std::io:Write` trait. In practice, that means that you can just treat the reader type like a standard iterable---because it *is* a standard iterable. When I talked about traits back in episode 9, I hinted at this kind of thing, with `std::iter::Iterator` specifically. Here's where all that power comes to bear: you can use the exact same set of iteration methods on an `XmlReader` as you can the built-in `Vec` type. Then, to do whatever transformations you need to do, you can just match on the `Event` type.

In Lightning, that translated to pulling the events out of the `XmlReader` instance, inspecting them to see if they were important for syntax highlighting (and doing the syntax highlighting if so), and then pushing them into an `XmlWriter` instance. To do this, I built a little state machine which determined what to do from each event coming out of the reader. If the event wasn't of interest for syntax highlighting, I'd just push it directly into an `XmlWriter`. If the events matched the magical pair of tags---`pre` and `code` with a language class attached---I could grab the *text* element, parse it, and hand it back, building a new `Element` out of the highlighted code. Then I could push *that* into the `XmlWriter`. And when I'd finished processing the whole thing, I could just convert it back into a string for further processing or write it to a file.

I've used very powerful XML parsing toolchains before; in particular I made heavy use out of the Python lxml tool a couple years ago when generating the HTML for HolyBible.com. Certainly if you wanted to do the kinds of things that gives you, you'd need to build on top of `quick-xml`. But, and this is important, you *can*. It's a great little library, with a very well-thought-out API. If you're working with XML or HTML, this is well worth becoming acquainted with. And if you haven't used a pull parser before, this is a great place to learn it. *I* hadn't ever done so, and I learned a ton from the experience. In fact, sometime in the next year or so, I end up implementing a parser in Rust, it will almost certainly be a pull parser. More on that when it happens.

## Closing

In short, if you're doing things with XML, `quick-xml` is a great bet---and it's also open to contributions, so if you find something it doesn't do well, you should file an issue or open a PR!

### Sponsors

Thanks to 

- Chris Palmer
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Philipp Keller
- Steven Murawski
- Raph Levien
- and Vesa Khailavirta

for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website.

### Follow

You can also find links to the docs, repository, and crate page for `quick-xml` at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it! You can also help others discover the show by rating and reviewing it on iTunes, recommending it in other podcast directories, or sharing it around on whatever social media you use.

I hope you've enjoyed this first crates-you-should-know episode, and I hope to put out at least one of these a month. I have a handful on my list, but I'd love to hear suggestions for more. You can shoot those my way, or respond more generally, in the threads for the episode on the Rust user forum or Hacker News, or by emailing me at hello@newrustacean.com.

Until next time, happy coding!
