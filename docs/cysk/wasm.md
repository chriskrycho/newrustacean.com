# Crates You Should Know: the `wasm-` tools

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is Crates You Should Know: the `wasm-` tools.

## `wasm`

### Background

First, a little background on WebAssembly, or “wasm”. I’ve mentioned it in a number of contexts on the show before, but I’ve not dug into the details at all.

WebAssembly is the direct descendant of a research effort called asm.js, which was an attempt to specify a subset of JavaScript that would work as a compile target for C or C++, specifically to allow games to run in web browsers. The aims of and possible uses for WebAssembly are much broader than just games though: it can be used for all sorts of performance-intensive applications, running as a complement to (rather than a replacement for) JavaScript in the browser.

### How it works

So how does that work in a technical sense?

WebAssembly is a binary instruction format for a stack-based virtual machine. Because it’s a low-level binary format, it can in principle run at native speeds – “in principle” because it does depend on the context it’s running in, of course. This low-level format serves a compile target for higher-level languages, where by higher-level I mostly mean “like C++ or Rust” rather than “like Ruby or Elm.”

WebAssembly can be run in all modern web browsers, and it can actually be implemented in existing JavaScript VMs. However, it is not *limited* to running in browsers or JavaScript VMs, and you definitely won’t see native performance if it’s implemented in an existing VM rather than having its own dedicated runtime. And it’s worth reiterating: the browser is just the first and currently-most-popular deployment target. I’ve already seen research on running it as kernel code – and to be clear, I mean as *normal*, Ring 0 kernel code, with neither a browser nor a JavaScript VM anywhere to be seen. There are possibly real long-term benefits to doing exactly that, too, though I’m not going to dig into them today.

But let’s step back and unpack that description a little bit.

- wasm is a *binary instruction format*. This is basically the same as other kinds of assembly languages, which are *instruction formats* as well. It’s a binary format for speed reasons: no parse-the-string-into-the-actual-instructions step required. However, there *is* a human-readable format, which is handy for understanding the output of your compiled code. 

- wasm is designed for a *stack-based virtual machine*. There are a variety of kinds of virtual machines in the world, and most of them you’re like to be familiar with (Java, C♯, Python, Ruby, JavaScript, etc.) have both a stack and a heap, along with garbage collection. The wasm virtual machine model does *not* have a heap, and it does *not* have garbage collection. This has a lot of implications for the programming model, which, again, I’m not going to dig into today. It’s enough for now to know that the constraints exist so you can have them in mind if or when you start actually implementing things in wasm!

It is important that I note that WebAssembly does have an explicit goal of being a viable compile target for languages which *do* have VMs in the future, but also chose initially to target languages with low-level memory models: C, C++, and Rust are the major source languages which target WebAssembly today. There are early experiments happening in other languages, too, but large-scale real-world application of those is probably a year or more away. And we’re here for the Rust side anyway, so let’s talk about how you’d actually go about targeting and using wasm with Rust today. Because you can, and it’s actually not all that hard!

## Using wasm

The first thing you should do is add the target, so you can compile to WebAssembly *at all*: run `rustup target add wasm32-unknown-unknown --toolchain nightly`. The name of the target triple tells you that you don’t know the architecture or the OS it’s running against; you know only the instruction set to use (in this case, `wasm` instead of say `i686` or `x86_64` or some such).

You can then manually run the compiler with the `wasm32-unknown-unknown` target specified, and you’ll see output in `target/wasm32-unknown-unknown`—with a `.wasm` extension instead of the usual binary. Then you’d need to pull that into your existing JavaScript environment with the right host bindings. In other words, you’ll need to basically write some JavaScript by hand to get everything doing exactly what we want and need to *use* this WebAssembly code.

So, it’s nice that we can actually build wasm! But clearly this isn’t especially useful output. We need some help from the crates ecosystem to be *productive* with it. And that takes us into the actual *crates you should know* for dealing with wasm.

### `wasm-bindgen`

The first crate you should know about is `wasm-bindgen`. If you’ve heard of the `bindgen` tool, this is like that, but for `wasm`. If you *haven’t* heard of the `bindgen` tool, it’s pretty much the C and C++ equivalent of what I’m about to describe for JavaScript.

WebAssembly itself only supports a very limited set of types: basically, short and long integers, and short and long floating points. The other “types” in the language are just functions or particular layouts of these kinds of types. But of course, in both Rust and JavaScript, we are essentially *always* dealing with types much more complicated than just integers or floats. `wasm-bindgen` does the work of generating mappings between WebAssembly and JavaScript that let you do things like pass a Rust string to JavaScript or deal with JavaScript exceptions in Rust. Without that, you’d need to write a ton of manual glue code—specifically, manual glue code in JavaScript that uses integers for pointer arithmetic and for indexes into reference types. That is not a lot of fun.

Instead of writing all of that code by hand—an error-prone process if ever there was one—you use `wasm-bindgen`:

1. Add it to your crate as a dependency.
2. Install the `wasm-bindgen-cli` tool.
3. Annotate both any `extern` JS items you want to use and also any Rust items you want to export with the `#[wasm_bindgen]` attribute. (You’ll also need to enable the `proc_macro`, `wasm_custom_section`, and `wasm_import_module` features at the top of your crate for this to work. That means a nightly version of the compiler: all of this stuff will eventually stabilize but right now we’re just figuring it all out!)
4. Compile your crate with the `wasm32-unknown-unknown` target, again using the nightly compiler. This will generate `target/wasm32-unknown-unknown/the_name_of_your_crate.wasm`.
5. Run `wasm-bindgen` on that output; you can specify the directory where you want the output. Now you’ll have a `the_name_of_your_crate.js` wherever you specified.



### `wasm-pack`

So it’s great that we can compile down to WebAssembly, but we’re still left with a pretty important question. How do we get that to the environment we’re consuming it from – *especially* in a way that means other consumers don’t have to worry about the fact that it’s Rust? After all, one of the things we want to be able to do is ship WebAssembly so that *anyone* can use it. If we’ve written some super speedy Rust/WebAssembly implementation that we want JavaScript consumers to be able to use easily, we want to make sure they *don’t* need to install Rust and understand *its* ecosystem. Wasm should just be wasm from their point of view.

Enter the `wasm-pack` tool! It’s a command line tool that actually abstracts right over `wasm-bindgen` and all the invocations we just talked through and builds a package that’s ready to ship to npm *for you*.

I talked through `wasm-bindgen` first because I think it’s important to understand what’s going on under the hood, and because you still need to do everything but the last step of running `wasm-bindgen` directly on your code. You replace that step by running `cargo install wasm-pack` and then running `wasm-pack` in the root of your crate, or somewhere else with the path to your crate as an argument. Then it runs `wasm-bindgen` for you on your compiled artifacts.

The result will be a compiled directory called `pkg`, with a `.wasm` file and a `.js` file which consumes it, alongside a `package.json` all set up and ready to ship to the npm registry. You can publish that to npm as long as you have Node and either the `npm` or `yarn` command line client installed! Then any consumer which understands WebAssembly—Node 8 or later, as well as an increasing number of bundlers like Webpack and all evergreen browsers—can run your WebAssembly!

## Closing

So that’s where things stand *today*. As I noted at the outset, this area of the ecosystem is moving incredibly quickly right now, so you should keep your eye on the updates from the wasm working group and watch these repositories. As neat as things already are here, they’re going to be a lot better by the end of the year!

Thanks to everyone who sponsors the show. This month’s $10-or-more sponsors included:

- Aaron Turon
- Alexander Payne
- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Derek Buckley
- Hans Fjällemark
- John Rudnick
- Matt Rudder
- Marshall Clyburn
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Paul Naranja
- Olaf Leidinger
- Oluseyi Sonaiya
- Ramon Buckland
- Raph Levien
- Vesa Khailavirta
- and Zachary Snyder

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for _this_ episode at <newrustacean.com/show_notes/news/rust_1_26>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!