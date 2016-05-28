# One year and counting

## Intro

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute show about learning the Rust programming language. This is *Episode 15: One year and counting*.

## News and Follow-Up

- First, thanks again for your patience. For one thing, the end of the semester is a busy time---I wrote well over 10,000 words on philosophy of science and religion from a Christian perspective in the span of six days last week! For another, we've just shipped a brand new mobile web experience at Olo, and getting that out the door was no small endeavor! The summer should be a little more settled, though it will still include some travel and some school.
- Second, and certainly of more interest to all Rustaceans: the first-ever RustConf has been announced! After last year's by all accounts very successful RustCamp, the good folks at Mozilla, Tilde.io, and a few other places are putting together a full-on conference in Portland, Oregon on September 10th, with a day of Rust training from core team members the preceding day. You better believe I'm submitting a talk proposal, and I'm hoping to be able to meet many of you there!
- Third, Rust 1.9 came out this Thursday! I'll talk more about that in just a minute.

## Happy Birthday

I originally planned to make this episode be a deep dive on smart pointer types like `Box`, `Vec`, `Rc`, `Arc`, and so on. And that episode is coming. However, as I started writing, I realized there is just a ton of *news* to cover, and so I thought I'd add another episode format to the show: summing up some of the big changes to the language, ecosystem and so on. These episodes will probably come every three to six months (and I have plenty to say on smart pointers, don't worry).

So for starters, happy birthday to Rust! Rust hit 1.0 May 15, 2015, and here we are a year later with a growing community, quite a few companies using Rust in production, and an awful lots of progress on the language, the standard library, and many increasingly well-regarded other libraries out there. In the lead-up to that, there were a series of posts on the Rust blog which gave a great overview of where things stand and what's coming up both for the core language and for the ecosystem. I'm not so much going to go those pieces in detail (I'll just link them in the show notes so you can take a look) as briefly summarize them and then interact with them.

I have to start by saying: I'm *really* excited about Rust. I mean, you knew that already from listening to this, but everything I'm about to talk about makes me extra enthusiastic. I also got to spend some time *writing* Rust for myself last night, for the first time in a couple months---with everything going on, I just haven't had a lot of spare cycles that way, but *wow* I enjoyed it.

### Rust proper

So let's talk about the news in the Rust world! As usual, I'll link to the relevant blog posts, RFCs, etc. for everything I'm about to discuss.

#### Rust 1.9

First up, Rust 1.9: just after the first birthday of Rust 1.0, we got the 9th minor language release. If you've watched any other language development process, this is nothing short of remarkable. Seriously: I know I gush about this a lot, but I would *love* to see other language communities adopt this kind of process. It means we get new things whenever they're ready, but without impacting our ability to keep working with the same codebase. Imagine getting a little bit nicer environment in Ruby or Python *every six weeks* but with nothing breaking. Imagine that with Java or C♯ or C++! It really is hard to overstate how big a deal this is.

Rust 1.9 added a couple niceties:

- Defining what a `panic!` should do. Now, you can specify whether it should kill the program with a full stack dump, or whether you actually want to "catch" the panic. This lets you handle, for example, cross-thread panics where you need something to fail on a secondary thread but deal with it, including finishing the stack unwinding, on the main thread. It also lays the groundwork for letting programs specify more generally how to handle panics in the future, including just aborting everything with no unwinding or stack trace.
- It also made the `#[deprecated]` annotation available to developers. This is a big deal for library authors: it lets them use the same machinery that's available to the standard library to say, "Hey, don't use this function/struct/enum/etc. Use this newer one instead; the old one will go away at the next major version bump." While we ultimately want to get to a point where library authors can introduce their *own* annotations, having this one be standardized from the core is a win regardless.
- There are some nice performance improvements in the compiler (and as we'll discuss in a moment, there are a lot more of these coming in the next year!).
- The ability to "specialize" the implementation of a given trait method has been put to use in the standard library for the first time. That functionality will become available in stable Rust soon, and when it does it'll allow a lot of neat performance improvements as well as some better ergonomics for trait methods in general. We'll talk more about trait methods and specialization in a future episode.
- There are bunch of nice improvements and additions to the standard library, as usual. You can check out the release blog post and release notes (which I'll link in the show notes, of course) for details.

#### MIR

Now, for some *really* nerdy fun: compiler details!

One of Rusts' biggest weak points since 1.0 (and before) has been its compile times. I've mentioned the "MIR," or "mid-level intermediate representation" effort briefly before, but it's *the* key change to the Rust compilation model that will speed up compilation times---because it's the piece that will let incremental compilation happen. If you're coming to Rust and your background is only with interpreted or just-in-time-compiled languages with a runtime (Python, Ruby, JavaScript, etc.), it might not be obvious that there are different approaches to compilation, only that the ahead-of-time compilation is different from the interpreted or just-in-time approaches. But in compiled languages, you can also distinguish between needing to compile the whole of some artifact every single time----whole-binary compilation---and being able to compile only the parts which have changed---incremental compilation.

One of the reasons that C C++ can be compiled faster than Rust in the ordinary course of application development right now is that the compilers supports *incremental compilation*: when you rebuild after making some changes, all major C and C++ compilers---Clang, MSVC, GCC, Intel, etc.---can recompile and re-link *only* the parts which have changed. So if you change one line in one module in a hundred-thousand-line program, you only compile that one module, and relink it. That makes ordinary day-to-day compilation proceed *much* faster than if you had to recompile the whole thing every time. (The same thing is true of languages like C# or Java, or really any languages on the .NET or JVM platforms, with a mixed model: which are ahead-of-time compiled to byte code and then just-in-time compiled for execution. For that matter, the same is true of Elixir, running on the Erlang VM.) Unfortunately, Rust has historically *not* supported incremental implication. But the new mid-level intermediate representation stage in the compilation process will make it possible, and that in turn will dramatically improve compiler performance in the ordinary case. Working on things like Rust itself, or Servo, or Dropbox's new cloud storage engine back end, or Tilde's Skylight tool for Ruby, or really any large project, will have a much faster development iteration cycle once it lands, and Rust will be much more competitive with other languages in this area.

So let's take a couple minutes and talk at a very high-level about how the Rust compiler behaves *today* and how it *will* behave in when the MIR changes ship. (That's *very* soon, because this effort has been in progress for about a year!)

For starters, it's worth note that Rust uses the LLVM compiler set. LLVM stands for "low-level virtual machine," a name which is somewhat confusing. The project came out of Chris Lattner's Ph. D. work, and forms the foundation of Apple's compiler toolchain---first for ObjectiveC, C, and C++, and now also for Swift. However, that "low-level virtual machine" provides a compilation target which can be and has been used for *lots* of languages, now include Rust. It provides very powerful optimization analysis itself, so when you're building a new programming language, you don't have to understand *every* detail of what's required for optimizing your code---especially on multiple compilation targets. (You still have to understand a lot, of course.)

Second, the basic flow of the Rust compiler goes something like this today:

0. Start with Rust source
1. Transform the Rust source to a high-level intermediate representation by parsing the source and desugaring it.
    - In the parsing step, the compiler reads the source and interprets it in terms of the tokens that make it up: the keywords, the operators, and
    - Desugaring takes the constructs in Rust which are "syntactic sugar" for something else and converts them into their more basic form. For example, we've talked about how the method call "dot" syntax is syntactic sugar before. If you had a struct named `Foo` and created an instance of it named `my_foo`, and called a method on it by typing something like `my_foo.bar()`, this would *desugar* into `Foo::bar(my_foo)`, because the first argument to methods is `self` and it's handed over implicitly.

    When the compiler has parsed the is done with this desugaring process for *all* the constructs like this (and there are a fair number), what you have left is the *high-level intermediate representation*.

2. The second step is to compile this high-level intermediate representation to LLVM intermediate representation (or IR). *Lots* of languages target this intermediate representation---anything compiled with LLVM, in fact. So if you're compiling C or C++ with Clang, or running the Swift compiler, or running the open-source .NET LLVM compiler for C# or F#, or using it as a front-end for Haskell or Java or Scala... well, you get the idea: it all ends up as LLVM IR. This IR is

    Today, in Rust, the step from the high-level intermediate representation to the LLVM intermediate representation is *huge*:

    - It does all the type checking. When you get one of those gnarly-but-super-helpful errors that tells you you tried to pass the wrong type to a function, it's coming out of this step in compilation.
    - It does the borrow checking. When the compiler complains that you tried to write data to something borrowed immutably somewhere else, it happens in this step.
    - It translates the parsed Rust source into the primitives used by the LLVM compiler. As you can imagine, given the list of languages that can use LLVM as a compilation tool, these are *much* lower-level primitives than the kinds of things you see in *any* high-level language.

3. The third major step is compiling the LLVM IR to the target binary. This has two major elements to it:
    - Optimizing! The LLVM compiler takes the LLVM IR and heavily optimizes it. You get the great performance you do out of Rust (or Objective C, or C++, or whatever else) compiled with LLVM because it has a really smart optimizer. (More on this in a minute, though!)
    - The optimization is target-specific: the compiler generates actual machine code for the specific architecture and OS kernel you requested, e.g. 64-bit Darwin for modern Macs, or 32-bit Windows NT for the stragglers still running XP, or any number of other targets.

So that's how the Rust compiler has worked historically. With the addition of the mid-level intermediate representation, there's one more step in the middle.

0. You still start with Rust source, of course.
1. The first step is still generating the high-level IR, and this hasn't changed at all: it's still just parsing and desugaring.
2. The second step is the new one: generating mid-level intermediate representation. All the type-checking now happens in the transition between the high-level and mid-level intermediate representations.

    This new mid-level intermediate representation (MIR) is something like a super-simplified version of Rust. It transforms many of Rust's high-level constructs into much simpler constructs. Basically, it turns all the Rust code you're familiar with into (and here I'm quoting from the writeup on the Rust blog), "a set of data structures and a control flow graph." This means that we can deal with everything from panics to match expressions to loop expressions to iterator operations in the same basic representation.

    That lets us figure out things like:

    - what actually needs to be recompiled when we change some of our code
    - more flexible rules around borrowing that are still equally safe but easier to write
    - how the relationship between panics and iterators might play out
    - when and how we can "drop", i.e. destroy, a given instance safely

    And all of that *before* the final compilation step where we drop down to LLVM IR.

3. The third step is generating the LLVM IR. But because we moved some of those steps up into the HIR to MIR transformation, less happens here than before. We still do the borrow checking in this step.

    But we also add a *new* step as part of transitioning to the LLVM IR: optimization! Because the mid-level IR gives us a better internal representation of the Rust code, we can actually do some Rust-specific optimizations that weren't possible when we just went straight from HIR to LLVM IR. Rust has a really expressive type system, as we discussed in episode 11; adding this step will let the compiler take much better advantage of that type system for optimizations. LLVM optimizations by definition have to work for *everything*, and languages like Ada and Rust are, well, just a *little* bit different. Getting specific will help a lot.

4. The last step is the same as before: LLVM compiles its machine code to the target binary, doing those really valuable but more general LLVM optimizations along the way.

So there's a lot there, but hopefully you understand Rust's compiler flow a little better and can see how the new mid-level intermediate representation will improve the day-to-day experience of using Rust for *all* developers, as well as make the language faster and more powerful going forward. One of the neatest things about this, in my opinion, is that if you go to play.rust-lang.org, you can actually *look at the MIR* generated by the compilation process, just like you can at the LLVM IR and the machine code. Super neat.

### Tooling

Two other neat things to highlight from the Rust blog that have coincided roughly with Rust's birthday.

#### Cargo

One of Rust's greatest strengths from the 1.0 time forward has been Cargo and crates.io, which together form its package management and distribution system. It's not an exaggeration to say these are best in class. I use pip and PyPI for Python, and Bower and npm for JavaScript, on a *very* regular basis, and I've interacted regularly with other package management tools like NuGet and RubyGems. Cargo and crates.io come together to make something *really* special. When I talked about how easy it is to use Rust even for just little command line tools you need, a *huge* part of what makes that possible is Cargo, which manages the whole build and distribution cycle---from creating a new project, to building it, to shipping it as a usable library or installable program, to installing those programs on your own system, to enhancing the functionality of the build tooling with tools like racer for code completion or clippers for code quality analysis. And it does all of that without turning into the kind of nightmarish or hideously arcane interface you might have come to associate with other especially powerful command line tools (I'm looking at you, Git).

Quoting from Yehuda Katz's blog post, the three pillars of Cargo's philosophy are:

1. Building, testing, and running projects should be predictable across environments and over time.
2. To the extent possible, indirect dependencies should be invisible to application authors.
3. Cargo should provide a shared workflow for the Rust ecosystem that aids the first two goals.

If you've spent time with, say, npm lately, you know how big a deal goal #1 is. Predictable builds across environments and over time is... huge. My team at work has been bitten by non-deterministic build issues with npm over and over again in the last few months, and it's  infuriating. Cargo makes strong guarantees that when you make a given build, you can reproduce it *exactly*.

The second goal just means that if you use a library which uses another library, you shouldn't *normally* have to worry about it. But what if they depend on say, versions 0.2.0 and 0.3.0 of a library? Well, things get messy. Cargo already handles this better than a lot of package managers, but there's room for improvement here, and one of the goals for Cargo in the months ahead is to make those improvements.

The last goal is a big one, and it's exactly what powers a lot of what I talked about back in Bonus Episode 4. If you have a powerful tool which has strong conventions that the community as a whole embraces, but which also lets you adjust it when those conventions don't match your specific needs... you have a recipe for huge success. It means you can write everything from an operating system to a tiny script-like tool with the same set of tools. Of course, it doesn't always mean you *should*: sometimes you should just use a shell script. But it's neat, and thoroughly empowering for the community, that you *can* do that.

#### `rustup`

The last bit for this episode is `rustup`, a new tool which will replace both the current rustup shell script, which is the default way to install Rust initially, and multirust, which is the current standard way to install multiple versions of Rust alongside each other. What makes this especially interesting is that the plan (and quite of bit of this already works) is for `rustup` to support not only installing and running different versions of the compiler but also managing cross-compiles. The day is coming when you might well be able to do the same kind of multi-target compilation with Rust you can do with Go or even Java---and in terms of making the language useful for the kinds of tools I discussed in Bonus Episode 4, that's *huge*.

It also makes it way easier to write and distribute things like compiled extensions for Python, Ruby, Elixir, etc. And, as a particular point of interest for me as a web developer, one of the planned target architectures for cross-compilation is the in-progress WebAssembly spec. Imagine a day when you could write Rust, compile it, and run it in your browser instead of writing JavaScript. For this web developer and Rust fanboy, at least, that prospect is incredibly exciting. I'm not-so-quietly hoping to be doing just that, just because I can, sometime late this year.

You can try `rustup` today by going to rustup.rs and downloading it. If you find bugs or run into issues with things that aren't documented yet, open an issue on the repository for it! And it's a small enough project that if you're looking for a good place to contribute to core Rust tooling, it's a good place to start.

## Closing

I hope you've enjoyed this new format episode looking at some of the ongoing language changes and news in the Rust world. I'll only do this kind of thing every three to six months (unless some *really* big news happens, of course), but especially since this is the main (and maybe the only) Rust podcast out there, I thought it might be useful. On that note, if you know of another Rust podcast, interesting video content or blog posts, etc., tell me about it! I'm very interested in promoting other parts of the Rust community!

In the next episode, I'll discuss those smart pointer types and what it looks like to use them in practice. The time after that, I have another interview headed your way from someone working on an interesting project with Rust!

### Sponsors

Thanks to Chris Palmer, Hamza Sheikh, Daniel Collin, and Vesa Khailavirta for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or one-off contributions at Venmo, Dwolla, cash.me, or Flattr.

### Follow

You can find links to everything I mentioned at NewRustacean.com. You can also follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it. It also helps others discover the show if you leave a rating and review it on iTunes, recommend it in other podcast directories, or share it around on whatever social media you use.

You can respond in the thread for the episode on the Rust user forum at users.rust-lang.org, or you can shoot me an email at hello@newrustacean.com.

Until next time, happy coding!
