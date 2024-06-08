# e029: I’m Out To C

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is e029: I’m Out To C.

## Sponsor: Manning

Manning is back sponsoring the show this episode! Once again they have a discount on some Rust *video content* they’re producing with Carol Nichols || Goulding and Jake Goulding, Rust In Motion—the very same we talked about when I interviewed them back in 2018! I’ve been saying for years that we need more video content: it’s a huge way people learn. Manning’s Rust in Motion is one of the first major video *courses* for Rust I know of (though I’m happy to report other video content is starting to appear as well). You can get it for 40% off at [deals.manning.com/new-rustacean](http://bit.ly/2OXnlEb) – there’s a link in the show notes. That link actually gives you 40% off of *anything* at Manning, including their book *Rust in Action*, which is in early access preview. Thanks so much to Manning for sponsoring the show and building some great video content with Carol and Jake!

## C FFI

Now, let’s talk about Rust’s foreign function interface, or FFI, and specifically its *C* foreign function interface. For further materials on all of this, you’ll want to take a look at [<cite>The Rust Programming Language</cite>’s materials][TRPL], the [API] and the [Reference’s][reference] writeups on `extern`, and the [nomicon] section on FFI. And speaking of Jake Goulding, his [Rust FFI Omnibus] a nice complement to those! I’ve linked them all in the show notes!

[TRPL]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
[API]: https://doc.rust-lang.org/1.33.0/std/keyword.extern.html
[reference]: https://doc.rust-lang.org/1.33.0/reference/items/external-blocks.html
[nomicon]: https://doc.rust-lang.org/beta/nomicon/ffi.html
[Rust FFI Omnibus]: http://jakegoulding.com/rust-ffi-omnibus/

One of the key selling points for Rust is that you don’t have to just *replace* your existing code in other languages, including in C or C++, but can interoperate freely with them via C APIs—since nearly all modern languages have a C foreign function interface, or FFI. I’m actually going to talk a fair bit about that in the *next* teaching episode, both for extending other languages with Rust and for Rust calling into other languages. For today, though, we’re looking specifically at C and C++, because the interop for those other languages builds on what we have here.

By definition, C has a C API. And C++ libraries *often* have C APIs or can be *expressed* as C APIs given a certain amount of finagling. But interacting with a C API means you have to write a bunch of Rust *bindings* for that API. A binding is a description to the Rust compiler of what the API looks like: a set of declarations of the types of both data structures and functions. It’s important that we get these right, because if we don’t, stuff *will* explode on us.

Throughout the rest of this episode, I’m going to teach you a *tiny* bit of C, because in order to work with Rust’s FFI tooling, you’re going to need to understand at least a *little* bit of C. To do it really *well* when starting from scratch for interop with other languages, you’ll need to understand more, but in many cases there are tools already that mean you don’t have to do *all* the work yourself, which helps a lot. I’ll say more on that in an upcoming Crates You Should Know!

### `extern`

The first thing we need to talk about is Rust’s `extern` keyword, which lets you define an <i>external</i> item: blocks, functions, types, and crates. I’ve linked to [the Reference’s writeup on `extern`][extern-reference], and it’s worth reading carefully.

You may recall that `extern` in the Rust 2015 Edition was required to declare all crates outside the current crate, with the `extern crate` syntax. This told the Rust compiler “Hey, this name isn’t part of this crate; you’ll need to look it up somewhere else!” Rust no longer needs that for *crates* in the 2018 Edition, as I covered in the news episodes where I talked about “path clarity.” However, we do still need the `extern` keyword, because there *are* functions and types we need to tell Rust to look up “outside”: no surprise given today’s topic, those are for FFI!

There are two forms where we use FFI. The first, which we’ll focus on in *this* episode, is `extern` *blocks*, which you use when calling out into something with a C ABI from Rust. The second is the `extern` modifier on an item declaration; we’ll talk about that in the *next* teaching episode, when we cover how you can use Rust from other languages.

In either of those positions, the `extern` keyword may optionally be followed by an ABI definition. An ABI is an <i>application binary interface</i>. Just as an API—an <i>application programming interface</i>—tells you as a programmer how to connect *your* code with *other* code, an ABI defines for the compiler how to connect different binaries together. Put another way: as an API is to source code, an ABI is to machine code. Some examples of the kinds of things an ABI has to define:

- how do data structures get laid out in memory?
- how are functions named when compiled?
- what is the processor instruction set to be used?
- how do system calls work in this particular context?
- how is the stack laid out?

There are more details than that; the point is that those machine-level details are super important and you have to define them somehow. Rust has its own ABI, of course, though it’s not something you ever have to worry about unless you’re working on the compiler itself. What’s more, Rust’s ABI can and does change. Other, “stable” ABIs exist, though, which *don’t* change over time, and that is useful when you need to interact with code that compiles down to that format.

You specify which ABI you want to compile against with a string literal after `extern`., e.g. to use the `stdcall` ABI on Windows, you’d write  The most common one is the C ABI!

There are two special cases to know about with `extern`:

1. When you write `fn foo()` (or indeed *any* item declaration), that’s equivalent to writing `extern "Rust" fn foo()`, specifying that this function will have the Rust ABI calling convention.
2. Because C FFI is far and away the most common, `extern` is short for `extern "C"`, both in blocks and in function declarations.

An `extern` block is exactly what it sounds like: a block which is marked with `extern` (and optionally an ABI string). The two big takeaways here are:

- *All* functions defined in an `extern` block are inherently `unsafe`, so even though you don’t have to and in fact *can’t* write `unsafe` on those declarations.

- For *all* FFI, you have to wrap the definitions in an `extern` block.

So, speaking of definitions… let’s dive in and talk about how you actually define `extern` functions and interact with them!

### The Basics

Any time we’re interoperating with something via a C API, we’re dealing with *functions*: there’s no other reason or way to interact across this kind of boundary!

We’ll start with a basic, apparently trivial example: calling out to C to perform addition. The C function signature is:

```c
int add(int a, int b);
```

The body does what you expect: returns `a + b`.

On the Rust side, we need to provide a *declaration* (but of course no definition) so we know how to call that function. We do that with an `extern` block and a function type with no body—somewhat like a trait method definition.

```rust
use std::os::raw::c_int;

extern {
    fn add(a: c_int, b: c_int) -> c_int;
}
```

Notice the types here: `c_int`, *not* `i32`. Now, `c_int` is nearly always an `i32`, but it’s not *guaranteed* to be anything of the sort, so we use these types instead. The types are defined in two places: `std::os::raw` and the `libc` crate. The definitions are the same; the only reason I know of to prefer getting them from `libc` is if you’re in an environment without `std`, e.g. on an embedded system. Using `c_int` will make it so that if you happen to be on a system where it’s *not* an `i32` and you call it with an `i32` without an explicit cast, you’ll get an error. Handy!

### Building C libraries with `build.rs`

Now, to make this work, you have to build the C you actually want to call. To do this, you’ll need to either use a tool outside of Cargo like Make, or use Cargo’s “build file” tool. For today, we’re just going to use the latter. In many cases, you’ll be using projects which already have their own build system, and will need to do some more work to tell rustc how to link those items into your app or library. The various references I mentioned at the top of the show all have info on how to do that.

For our purposes, I’m going to use this as an opportunity to talk through Cargo build files!

If you define a `build` key pointing to a Rust file in your `package` config in the `Cargo.toml` file, Cargo will compile and execute that Rust program before building the rest of your program. Conventionally, you can simply supply a file named `build.rs` and Cargo will do the same thing automatically, without even needing to specify the `build` key. If your build process is sufficiently complicated, you can just set up a `build` directory, make the root `build/main.rs`, and have a full Rust program in that directory which your `build` key points to. It’s quite powerful!

You can also specify `[build_dependencies]`, which will only be used by the `build.rs` file. In the show notes for this episode, you’ll find a sample `build.rs` file which is using the `cc` crate to build a small bit of C code and set it up in the right spot to be statically linked into our program. The `cc` crate does the hard work of abstracting over the differences between C compilation environments on different platforms, so you don’t have to worry about working out those different invocations for `gcc` and `clang` and `msvc` and so on.

### Structs

So that’s the *basics* for our `add` function, though I’ll have more to say about it in a moment.

Let’s say we had a very traditional `struct` type in C, just a plain old record—we’ll go with a `Point` because that’s such a common example. Let’s also imagine we wanted the ability to *translate* the point, to change the coordinates from one location to another. In C, we’d write that like this:

```c
typedef struct Point {
    float x;
    float y;
} Point;

void translate(Point * point, float byX, float byY) {
    point->x += byX;
    point->y += byY;
}
```

Writing `Point* point` means `translate` requires a *pointer* to a given `Point` piece of data (but it makes no guarantees that there’s actually a `Point` behind it, only that C will treat the data layout as consistent with a `Point`). The net of this declaration is that we are taking in a *pointer* to a `Point` and can freely do whatever we want to the memory behind that pointer—quite *unlike* in safe Rust. We could also here mutate the pointer itself, pointing it to some *other* point in memory. If we wanted to make it so you couldn’t change what the pointer pointed to, we’d write it `Point *const point`. If we wanted to make it so we couldn’t change the *contents* of the point but could change what it pointed to, we’d write `const Point * point`. If we wanted to make it so we couldn’t change *either*, we’d write `const Point *const point`. Unfortunately, C would *also* let us cast away all of that attempted safety and just do whatever we wanted. In this case, we do the right thing: we just update the values in the `point`.

Now we need to write those same definitions in Rust. First, the `Point` type. This looks pretty much like what you’d expect: <!-- just read the following paragraph -->

```rust
use std::os::raw::c_float;

#[repr(C)]
pub struct Point {
    pub x: c_float,
    pub y: c_float,
}
```

It’s *almost* just a normal `struct` type—`pub struct Point` with public properties `x` and `y`, both of type `c_float`. So far so normal. However, we also need to add an attribute, `#[repr(C)]`, to the struct declaration. This makes it so that Rust lays it out using the C ABI’s definition of how structs are laid out, rather than how Rust chooses to lay them out. Once we’ve added that, though, this definition is exactly the same as the C one and we’re good to go with interop.

Next up, the function definition, but with raw pointers instead of references. I skipped over this in my discussion of `unsafe` code in Rust, because I knew we’d hit it in *this* episode: Rust has a similar notation, in the form of *raw pointers*, written `*const` and `*mut`. In Rust we’d write this same first argument `point: *mut Point`, since we’re passing a pointer pointing at a thing which will be mutated. Notice an important difference here: when we write `*const` in C, it means “you can’t change the pointer”; in Rust it means “you can’t change the thing the pointer points to.” The full function signature for this in Rust, then, is:

```rust
use std::os::raw::c_float;

#[repr(C)]
pub struct Point {
    pub x: c_float,
    pub y: c_float,
}

extern "C" {
    pub fn translate(point: *mut Point, by_x: c_float, by_y: c_float);
}
```

The other arguments here are straightforward: `c_float` for the same reason we used `c_int` in the addition example.

### Safe wrappers

You *can* expose an unsafe interface that just provides direct interop with C functions in your code. However, this means that *everything* which uses that code has to uphold its invariants! As I discussed back in Episode 27, the Rustic way of doing things is to wrap our `unsafe` functions in safe Rust functions, which check that all the invariants are correctly upheld. That way the *vast* majority of your code—even your code that relies heavily on external C API code—can just be normal safe Rust; we *isolate* the un-safety and the complexity and the testing burden.

So, taking that approach with the examples I gave above, we might have the same `extern` definition for the C functions, but wrap them in a non-exposed module—maybe called `ffi`—and only expose our own safe wrappers. That would be something like this:

```rust,ignore
use std::os::raw::c_float;

#[repr(C)]
pub struct Point {
    pub x: c_float,
    pub y: c_float,
}

mod ffi {
    use super::Point;
    use std::os::raw::{c_float, c_int};

    extern "C" {
        pub fn add(a: c_int, b: c_int) -> c_int;

        pub fn translate(point: *mut Point, by_x: c_float, by_y: c_float);
    }
}

pub fn add(a: i32, b: i32) -> Option<i32> {
    if i32::max_value() - a >= b {
        unsafe { Some(ffi::add(a, b)) }
    } else {
        None
    }
}

pub fn translate(point: &mut Point, by_x: f32, by_y: f32) {
    unsafe {
        ffi::translate(point as *mut Point, by_x as c_float, by_y as c_float);
    }
}
```

The `add` definition here tries to do something *somewhat* reasonable when dealing with the possibility of overflow—not a *great* solution (and not even a *complete* solution, since you could underflow, too!), but we need *something* here. The reason is that as silly as this kind of check might seem—about as calling out to C for addition is!—even something like addition which might seem to be trivially safe… isn’t.

While it might seem that something like addition is trivially safe, it turns out to be *mostly* safe. The behavior of overflow for signed integers is*not defined* for C. In Rust, it *is* defined, by [RFC #0560][RFC-0560]: in modes where `debug_assertions` are enabled, an overflow will cause a panic; in modes where those assertions are not enabled (i.e. release mode), Rust wraps them by [two's complement][tc]. The net of that is that even something this simple can have unexpected results when calling across the FFI boundary.

[RFC-0560]: https://rust-lang.github.io/rfcs/0560-integer-overflow.html
[tc]: https://en.wikipedia.org/wiki/Two%27s_complement

This is a silly example, of course, but it shows how you can provide a safe wrapper for a case where C's implementation differences *might* actually matter to you… and it also serves as a reminder that C’s implementation differences are likely to matter in more places than you might initially think.

In the case of the safe wrapper for `translate`, we don’t need to do anything except call `unsafe { translate(...); }`, because we don’t have any particular constraints we have to uphold other than that we legitimately have a non-null pointer to pass in—and pointers are *always* non-null when coming from safe Rust. But by wrapping it up this way, we’ve still made it so our consumers don’t have to deal with the un-safety. Here, I’ve chosen to add the explicit casts as a way of communicating to anyone reading the code what I’m doing, but none of the arguments actually *requires* it.

Finally, it’s worth clarifying that our careful use of safe Rust interfaces does *not* mean something can’t go wrong here through our use of unsafe code. The C code itself could still do something horrible—cast the pointer to `void *`, point it at other junk memory, or even call `free` on it. Lots of things can go wrong in C (or indeed in other languages with C APIs) that Rust by definition doesn’t have control over. When we provide a safe interface like this, we are not saying anything at all about the actual behavior of the C API we’re calling into. The *only* thing we’re doing is saying that we’re providing data that satisfies the invariants the C API requires.

## Conclusion

Okay, so that takes us through the basics of C FFI in Rust. There’s a lot more to say, though! In the next teaching episode, we’ll cover dealing with enums, types like strings and vectors and arrays and slices, ownership issues, and union types. After *that*, we’ll dive into talking about how you can use Rust *from* other programming languages! Along the way, there will also be a Crates You Should Know about bindgen.

Thanks as always to *everyone* who sponsors the show. This month’s $10-or-more sponsors included:

- Anthony Deschamps
- Ryan Osial
- Ola Fadeyi
- Soren Bramer Schmidt
- Rob Tsuk
- John Rudnick
- Chip
- Brian McCallister
- Jerome Froelich
- Andrew Dirksen
- Nick Stevens
- Daniel Mason
- Daniel Collin
- Joseph Schrag
- Jako Danar
- Embark Studios
- Peter Tillemans
- Jonathan Knapp
- Adam Green
- Michael Mc Donnell
- Matt Rudder
- Graham Wihlidal
- Benjamin Manns
- Jeff May
- Martin Heuschober
- Oluseyi Sonaiya
- Scott Moeller
- Nathan Sculli
- Behnam Esfahbod
- Evan Stoll
- Arun Kulshreshtha
- Raph Levien
- Ramon Buckland
- Nick Gideo
- Daniel Bornkessel
- James Hagans II
- Dan Abrams
- Paul Naranja
- David Carroll
- Bryan Stitt
- Nicolas Pochet
- Johan Andersson
- Jason Bowen
- Alexander Payne
- Zach Peters

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/e029/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!
