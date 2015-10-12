# e01: Document All The Things

## Intro
Hello, I'm Chris Krycho and this is the New Rustacean podcast, a 15--20 minute
show about learning the Rust Programming Language. This is episode 1.

## Fun News
MSVC integration (the first pieces of which landed in 1.3) are coming along in
the 1.4 beta (should land at the end of October). Huge for being able to use it
with existing C codebases on Windows (like, for example, in one of my ongoing
contracts).

## Docs

### Why do we care?

  - Just plain handy. No need to use external tools (Sphinx, Doxygen, JSDoc,
    etc.) because it has language-level support.
      + Predecessors: Python, Java
      + Modern, similar tooling: Elixir, Julia

  - Shapes the community: doesn't guarantee anything, but it makes good docs
    *likelier*.

  - Standardizes the docs. Python docs are all over the map---wildly different
    layouts, themes, etc. Finding things is hard

### `rustdoc`
  - Documentation comment types
  - Module-level docs
      + different kinds of documentation comments
  - Item-level docs
      + functions
      + structs and enums
      + members
  - How to use the command, and what you'll get when you do.
      + everything needed for that particular module in the format you see in
        package documentation online
      + Only publishes public items
          * More on public/private next week

### `cargo doc`
  - auto-builds full project docs and drops them in `target/doc`, just like
    `rustdoc`
  - the difference: automatically includes all public elements in a given
    project, so you don't have to run `rustdoc` manually over and over again!
  - limitations for full sites
      + `rustbook` tool
      + Rust and Cargo sites built using other tools to pull the pieces together

### What's next?
