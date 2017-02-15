# `clap`

Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 show about learning the Rust programming language. This is *Crates You Should Know: `clap`*.

## The crate

### Background

When I started building Lightning, I knew that part of what I would need would be a really great command line interface, and that it would need to be robust enough to handle not just generating a site but also rapidly creating posts, pages, etc. In my previous setup, I'd actually created a custom tool to do all of those things, and other site generators (like Jekyll) include this functionality.

In Rust, you have three major options for handling this kind of thing.

1. You can just directly access the arguments handed to the process, using the `std::env::args` function, which hands you back an iterable struct of the arguments.
2. You can use `docopt`, a Rust implementation of the docopt command line interface language (from Python), by the same guy who built the Rust regex library and the `ripgrep` Rust grep tool. It's a great tool, and I might come back to it at some point in the future.
3. You can use `clap`: a *command-line-argument-parser* (where the name is an acronym from the description: c-l-a-p). This is our focus for today, and it's *very* different from `docopt` in a lot of ways.

I don't recommend using the args directly unless perhaps for a very small toy bit of code. Using `docopt` or `clap` is a much better choice: both give you help messages essentially for "free" once you've done the work of setting up the structure. Both can also give you basic shell completion for bash, and clap can generate completions for zsh, fish, and PowerShell as well. You also get a lot of mileage out of community-tested and -embraced tools.

One other qualification before I dive into talking about `clap`: I picked it not because I think it's *better* than `docopt`, but because it was the one that meshed best with how I like to approach this problem, and so was the one I started using with Lightning.

### The API

The usage pattern for `clap` is straightforward: you define a configuration it should use, and at runtime it parses the args wherever in your program flow you take that configured struct and run the handy `get_matches()` method. Usually, that's right at startup, of course. `get_matches()` takes the arguments passed to the program and converts them into a data structure you can readily convert into whatever *internal* data structures you need for running your program. There are three major ways to configure `clap`.

First, you can use a normal (and quite elegant) Rust API to build it directly inline. This is the tack I took initially with Lightning, because it was the most familiar to me after having used Python's `argparse` library in the past. And in fact, in many ways `clap` feels like a spiritual successor to that library, so if you've used it and like it, you'll appreciate `clap` a lot---not least for the ways it *improves* on that earlier model. In this mode, you get to use the *builder* pattern, a concept I may have mentioned before but which I now realize deserves its own full episode.

From a user's perspective, though, you just create an instance of the type by writing `clap::App::new()` and then supplying further calls on that instance. The methods are set up so that you can just chain the calls one after another---you don't have to set up any intermediate variables. Other methods include things like `arg`, `subcommand`, `version`, `author`, `about`, and so on. There are also the `Arg` and `Subcommand` structs, along with several other helper structs. `Subcommand` instances are just more `App` instances, but attached to a parent `App`, so they have all the same customizability and capabilities as the main `App`. `Arg` instances let you specify the name (and a short variant) of the argument, whether it's optional or required, whether it's positional, help text for the argument, even things like whether the argument is required *unless some other condition* holds. They're basically as configurable as you could want. You can also group `Arg`s with `ArgGroup`s.

Second, you can write a YAML file specifying the various options and their relationships. These give you a *subset* of the capabilities you have in the full library API. Mind: it's an extremely full-featured subset. Though I started off with the Rust API for Lightning, I ended up switching over to the YAML version because it does everything *I* need. (Note that the YAML version is behind a feature option in Cargo---another thing I'm pretty sure I haven't covered, but which should probably get its own short episode.) I'll link both my Lightning code and, more importantly, the example in the `clap` codebase, which covers *all* the features the YAML variant supports.

Third, you can use a macro, `clap_app!`, which gives you a handy short-hand syntax which sits somewhere between the YAML and the library API in terms of ergonomics. It's definitely briefer than the full API to use, and looks closer to the kinds of things you'd write in the YAML form. It will, by dint of being a macro, be much more difficult to see where you got things wrong if you mess up, compared to the full library API.

I should also add here that there are a bunch of other convenience macros you can use to get information about the crate at compile-time even in the standard API approach---tools like `crate_authors!`, `crate_description!`, and `crate_name!`, which give you, well, the crate authors, description, and name respectively. Those let you keep the name, description, and author fields of your help output in sync with what's in your crate automatically. Don't-repeat-yourself and all that jazz!

One other thing I want to say about `clap`: its documentation is simply superb. The index of the automatically generated docs includes a lengthy example of how to use the library in each of the three ways mentioned above. There are also multiple examples of using the app in the `examples` directory in the repository, which I believe combine to cover every feature the library has. I had a question at one point about something I didn't perfectly understand from the documentation, so I went and looked and exactly that case was covered by one of the examples; it solved my question entirely. That was a *fabulous* experience, and I wish more libraries did it. It's a big inspiration for me, in fact: I won't be calling Lightning "1.0" until I have that same degree of coverage in docs.

There's even a work-in-progress online *book* on creating command-line applications, using clap. Note: when I say "work-in-progress" I mean it: the site itself describes the material as "pre-alpha" and many of the chapters don't yet exist. Still: there's a lot of great material here.

## Closing

In short, `clap` is a *fantastic* utility library for building command line tools, and it's well worth your time! It's also actively developed, in the open on GitHub, so you can and should contribute if you find a gap or a bug!

### Sponsors

Thanks to 

- Chris Palmer
- Christopher Giffard
- Matt Rudder
- Ben Whitley
- Peter Tillemans
- Philipp Keller
- Steven Murawski
- Raph Levien
- and Vesa Khailavirta

for sponsoring the show this month! You can see a full list of sponsors in the show notes.

If you're interested in sponsoring the show, you can set up recurring contributions at Patreon.com/newrustacean, or give a one-off contribution at any of a number of other services listed on the show website, or if you're a company interested in advertising to developers, please email me!

### Info

You can also find links to the docs, repository, and crate page for `clap` at NewRustacean.com. You can follow the show on Twitter @newrustacean, or follow me there @chriskrycho. If you enjoy the show, please tell somebody about it! You can also help others discover the show by rating and reviewing it on iTunes, recommending it in other podcast directories, or sharing it around on whatever social media you use.

I'd love to hear your feedback, as well as suggestions for topics, interviewees, and so on, in the threads for the episode on the Rust user forum or Hacker News. I also love getting email---you can send me a note at hello@newrustacean.com.

Until next time, happy coding!
