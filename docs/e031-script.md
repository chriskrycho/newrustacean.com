# e031: FFI Deep Dive!

## Intro

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is e031: FFI Deep Dive!

## Sponsor: Parity

Before we dive in, Parity is back sponsoring the show—because they want to hire you to come work in Rust with them!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## Colorado Gold Rust

One quick bit of news from the community: the Colorado Gold Rust call for proposals is open, so if talking to and hanging out with a bunch of Rustaceans (almost certainly including me!) in Colorado this fall sounds like fun, please submit a proposal.

## Rust for FFI

Okay, now, let’s dig in. A couple episodes ago I talked about doing C FFI from Rust. Today, we’re going to flip that around and see how you can use Rust for FFI from *other* languages. Languages *plural* because everything I cover here is equally relevant to using Rust as the FFI layer for, say, Elixir or Python as it is for integrating Rust into an existing C or C++ codebase. And in that respect, this is actually *more* interesting than just calling with a C API from Rust: because it lets Rust provide safe super-powers for other languages!

There are two basic kinds of things we need to cover today:

- exposing Rust functions and data types *to* other programs via the `extern` mechanics we talked about last time
- and handling richer data types both *from* other languages in Rust, still using those `extern` mechanics but also adding in the idea of *opaque types*

Throughout all of this, I’m very indebted to Jake Goulding’s FFI Omnibus.

### Making Rust functions available to other programs

The first of these is essentially the inverse of what we talked about in e029. In that case, we were defining Rust signatures for types or functions that are coming in from other languages—specifically, from C APIs. How do we go the other direction, though? Let’s say we were working in a language like Elixir, or Ruby,  or JavaScript, or Python, and had some hot loop that we needed to go *much* faster? Because Rust can exposes a C foreign function interface, you can use Rust instead of C for native extensions to those languages!

We’re going to work through this in three parts: numbers, strings, and other more complicated types. For anything more complicated than a number, we’ll talk about what you have to do to be memory-safe as well. Finally, we’ll wrap up the episode by talking about <i>opaque pointers</i>: a very important tool for safe interop!

You should, once again, definitely take a look at the sample code in [the repository for the show][repo]: I’ve made the show notes work as a library which you can dynamically link against from anything expecting a C ABI. In particular, you’ll want to look at the code in the `e031` directory, which includes a `mod.rs` and associated `e031.c` and `e031.h` files, and at the `build.rs` file in the root, which includes a *very* simple setup for building those C files into an executable you can run, showing that this stuff actually works. To try it, check out the code and run `cargo build`; you’ll get a binary file in the root of the crate called `link-against-rust` and you can run it to see that it does what I claim it does! Unfortunately, that only works on Unix-like operating systems because I don’t have a good way (or the time!) to set this up on Windows—but you should be able to make it work on the genuinely great and constantly-improving Windows Subsystem for Linux.

[repo]: https://github.com/chriskrycho/newrustacean

### Configuring a project

First, let’s talk through how you actually expose *anything* for *consumers* of a Rust-based C-style API. To make a function available over Rust’s C FFI, we have to do just a couple things:

1.  We need to update our crate definition a bit. By default, Rust library crates are build as `"lib"` crates, which means they’re *Rust*-compatible, but they could be any of a number of formats; the compiler gets to decide which. However, `"lib"` crates are *not* C-compatible. So in our crate definition, in the `[lib]` section, we write `crate-type = ["cdylib"]` (assuming you’re on at least Rust 1.10; before that you had to use just `"dylib"`). This crates a dynamically linked library artifact which is designed to be linked into a C program. If you run `cargo build` after doing this, you’ll see a dynamically-linkable library file for your OS: `.so` on Linux, `.dylib` on macOS, `.dll` on Windows.

    You may have noticed that I said to write a TOML *array* for the `crate-type` value. That’s because you can actually pass *multiple* values in there. So if you want to build a library for consumption *either* as a dynamically-linked library for C consumers *or* for static linking into a Rust program… you can do that! You just write `crate-type = ["lib", "cdylib"]`. Then if you do `cargo build` and look in `target/debug`, you’ll see both an `.rlib` file and the appropriate dynamically-linkable library file for you platform.

2.  As with last time, we’re going to use the `std::os::raw` or `libc` libraries to make sure we’re writing types that do what we need them to, rather than Rust’s own native types. In general, `libc::<some_c_type>` is just another name for `std::os::raw::<some_c_type>`. Remember: the big difference between the two is that libc doesn’t require std, so if you’re working in a context where you need to build in `no_std` mode—embedded environments, for example.

3.  Mark each function we want to make accessible to non-Rust callers with the `#[no_mangle]` attribute. This tells the Rust compiler *not* to do the normal name-mashing it does, where it takes all the information about a given function—including its module, generic monomorphization, etc.—and turns that into a distinct name for each way the function can be invoked.

    If you pull down the source code for the show and do a `cargo build`, you can actually inspect the output for the `cdylib` it now creates using the `objdump` or `nm` tools, which let you look at the symbols in a given binary file. You’ll notice a bunch of names that look *sort of* like names from the Rust standard library… but mangled! You’ll also notice a couple functions whose names *aren’t* mangled if you search down through: `add_in_rust` and so on. These are the functions you can see in the show notes for this episode, which are exported publicly with the `pub` and `extern` modifiers and marked with the `#[no_mangle]` attribute.

That’s it for the basic *mechanics*.

### Numbers

Now, we can start seeing what it looks like to put this into practice, with some simple numeric examples. Happily, numbers are pretty easy. In fact, for numeric work, it’s safer and easier extending other languages with Rust than reaching out to C APIs *from* Rust, because in this case, Rust is in control of everything—importantly, including things like overflow. We’re not doing unsafe things here! (That’ll be a bit different when we talk about more complicated types in a minute, but… we’ll get there in a minute.)

I’ve put in the show notes a super simple example of this: addition. We just write:

```rust
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    a + b
}
```

There shouldn’t be anything surprising about that if you listened to e029. We’re using the `c_int` types to make sure we’re using the types of the sizes C consumers expect—whether those C consumers C, C++, Elixir, JavaScript, or whatever else. We make it `pub` so that external consumers of the crate can see it. We mark it `extern "C"` so that it’s guaranteed to have the right ABI, as we talked about in e029. We mark it with `#[no_mangle]` so that the name ends up in the format expected for linking in as a C-compatible library. And, for this simple case… that’s it on the Rust side! On the consuming side, you’ll also need to *declare* that with a C declaration:

```c
extern int add_in_rust(int a, int b);
```

Otherwise your C library or binary simply won’t compile because it won’t know that something outside itself is supposed to be supplying that symbol. It’s marked `extern` as well, but in the C case that means the function has <i>external linkage and storage duration</i>. The details of that aren’t really relevant for us, but they *are* worth understanding if you’re doing a bunch of this.

These kinds of numeric calculations are a prime place where Rust can be a super helpful way of speeding up a scripting language. If you have some large, computationally-intensive task you need to do, that’s reasonably isolated from other code, and your profiling shows you that there’s a hotspot there, writing a native Rust extension to do that work can often be a perfect solution.

Of course, in many cases, you’re dealing with something like an array of data—so we’ll need to figure out how to pass that kind of thing across the FFI boundary!

### Strings

We’ll start with one of the most common vector-based data structures: strings! There are two ways we need to be able to deal with strings: when *Rust* owns them and is handing them off to a C API, and when *the C API* owns them and Rust is dealing with them. For this we’re going to use what seems like a pretty simple example: concatenating two strings. As anyone who has worked with strings at a low level knows, though, doing this *correctly* and *safely* is harder than it seems—which I was reminded of as I wrote this episode. (Rarely have I been so grateful for Rust’s string handling!)

We’re going to have a function, `concat_strings`, which takes in two strings from a C ABI and returns another string back to the C API. (Remember, this works equally well for any language with a C API.) I’ve put the full code sample in the show notes, and you should read it. Here, I’m just going to talk through the function signature and the things we need to do to make this work correctly. On the Rust side, the function signature is:

```rust
#[no_mangle]
pub extern "C" fn concat_strings(first: *const c_char, second: *const c_char) -> *mut c_char {
    // ...
}
```

The reason we’re taking pointers to `c_char`s is that C doesn’t actually have a notion of strings as such—it just has arrays of characters. A “string” is just a pointer to such an array, terminated by a null character. Now, since we’re taking in data from C, this is totally untrustworthy. We need to start by checking that the pointers aren’t `null`, and then we need to convert the C-style array of `char`s into a type Rust can work with safely. In an `unsafe` block, we’ll `assert!` that both pointers are not null, then use `CStr::from_ptr` to convert the pointer into Rust’s special type for C strings. `CStr` is the reference type, and `CString` the owned type, analogous to `str` and `String` respectively. And once you have a `CStr` or `CString`, you can convert it into a regular `String`—but with a cost for checking that it’s valid UTF-8. We don’t need to do that here, because we’re just sending it right back across the FFI boundary, but if we were going to work with the string internally in Rust code beyond this, that would be worth doing.

Here, we take those two `CStr`s and turn them into slices of `u8`, drop each one’s terminating null byte, and then concatenate them and a single null byte with `std::slice::concat`. Then we create a new, owned `CString` from the result. This allocates the space on the Rust side for the string, which we’ll need to deal with in a minute. For sending it back across the boundary, though, we’ll just call `.into_raw()` on the `CString` and return that. That turns it into the `*mut c_char` that C needs to interact with it.

Two important notes here:

1.  This isn't actually the *right* way to do this in terms of idiomatic C APIs. Normally we would want to ask the caller to pass in a pointer to a buffer with enough space allocated for the concatenation, and would return an error code with `0` for success and a different number otherwise. However, it’s a useful example, so I did it this way anyway!

2.  We’re returning a mutable pointer because we’re handing it back to a C API to do with as it will. (Even if we declared it as `char const * const`, the caller could choose to ignore it. Modern C compilers will *warn* about this, and you should listen… but they won’t *stop* you.)

Now, the last thing here: Rust allocated the string, so Rust needs to free it. If you call `free` on the C side, or use the normal garbage collection mechanics in a language with a garbage collector, you will cause a memory leak, because the internal layout of a `CString` is a `Box<[u8]>`, and we need to make sure that gets deallocated correctly. We handle this by providing a `free_rust_string` function, which uses `CString::from_raw` to make sure everything gets dropped appropriately when it goes out of scope… which it does *immediately* because we don’t do anything else with it.

This is the basic pattern for *all* rich data types going back and forth across the C FFI boundary in Rust. Arguments coming in as pointers need to be checked that they’re not `null`, since `null` pointers are not part of safe Rust. And when we allocate on the Rust side, we also need to *free* on the Rust side. These kinds of things are constraints you must document, and document *loudly* for any consumers of your library… including yourself! This is the key with all unsafe code, including by definition all FFI: you have to do the work that the Rust compiler does for you with safe code.

### Structs

Moving on to structs, basically everything I just said about strings applies identically—which shouldn’t be a big surprise since `String` *is* a struct. Imagine my go-to example of a `Point`, with `x` and `y` fields. We can define that so it can be shared across the FFI boundary freely, by writing it with `#[repr(C)]` on the struct definition. Then we can write the corresponding C struct definition, and use it on either side of the language boundary.

In this mode, we can start by declaring a `Point` on the C side and initializing its values to whatever we wish. Then, if we want to translate it from one location to another but on the Rust side, we can write a `point_translate` function just like our `concat_strings` function.

```rust
#[no_mangle]
pub external fn point_translate(point: *mut Point, by_x: c_float, by_y: c_float) {
     
}
```

That function need to take the mutable pointer, check that it isn’t null, and then dereference it—which is `unsafe`!—to get an actual Rust reference to a `Point`. Then we can perform the translation in a normal Rustic way, like `point.translate()`. We don’t have to do anything special once we’re done, because C doesn’t have any notion of ownership, and from the Rust perspective we relinquish the borrow as soon as we’re done. But of course, we can also simply mutate it in place on the C side (and the code sample does exactly that!).

#### Opaque pointers

This is *not* optimal. We’re back to having shared mutable state, and that *across a language boundary*. At that point, we pretty much might as well just write it in C! What we can do instead is define <i>opaque pointers</i>. We expose the *existence* of the type, but we *don't* make it `#[repr(C)]` and we *don't* expose its internal fields at all. When we declare it on the C side, we declare it with no details at all:

```c
typedef struct opaque_point opaque_point_t;
```

Then on the Rust side, we write our `OpaquePoint` exactly like the `Point` we wrote earlier, but the fields can be private… and there is no way to construct one directly on the C side, and no way to *modify* it on the C side. Instead, we add and expose another function on the Rust side. It has the signature: <!-- read the function *signature* only, skip to the next paragraph for the internals -->

```rust
#[no_mangle]
pub fn opaque_point_new(x: c_float, y: c_float) -> *mut OpaquePoint {
    Box::into_raw(Box::new(OpaquePoint { x, y }))
}
```

To make it return that mutable pointer to an `OpaquePoint`, we create the instance like we usually would in Rust, wrap that in a `Box`, and then call the associated function `Box::into_raw` on the result. This gives us the pointer we need. We’ll also need a new function that we can use to describe the point’s value, since we don’t have access to its internals. We can combine a bunch of the other pieces we’ve seen already: implement `Display` for the type like any other, and then expose a `opaque_point_describe` function which takes a pointer to an `OpaquePoint`, then generate a `CString` from a the display formatter and return a reference to that `CString`.

Finally we’re responsible to call free for any string we get that way *and* for the `OpaquePoint` when we’re done with it. We already have the `free_rust_string` function from earlier. Our `opaque_point_free` function will invert the process by which we got the pointer. As with strings, if it’s already `null`, we ignore it. If not, we do the `unsafe` operation `Box::from_raw`, and then do nothing; it properly executes its `Drop` implementation as soon as it goes out of scope, cleaning up both the allocated `OpaquePoint` *and* the `Box` wrapping it.

Opaque types like this are a *super* useful primitive for making sure that we can expose Rust types to other languages while still actually maintaining all the invariants required for Rust to be *useful*.

### Other rich data types

Before we wrap up, I want to survey three other types you should understand for FFI: unions, enums, and tuples.

#### Unions

C also has a kind of type that *safe* Rust doesn’t: unions. A union is kind of like an enum: it’s a single type which can represent a variety of different shapes. However, Rust’s enums are checked by the compiler: they are <i>tagged</i> and can be differentiated based on that tag. For example, you can think of the `Result` type as being a *union* of the two types it contains, with a tag specifying whether it’s the `Ok` value or the `Err` value. You write a union like this:

```rust
union IntOrFloat {
    int: i32,
    float: f32,
}
```

Once you’ve written that, you can create a union with syntax like struct initializer syntax, but you can only assign one of those variants: `IntOrFloat { int: 42 }` *or* `IntOrFloat { float: 0.5 }`. Once you have a union, accessing its internals requires an `unsafe` block, because you can access `int` or `float` (in this example) regardless of which one is actually defined… and so you can hose yourself very badly. The show notes have a sample implementation of an `Either` union type, which is very similar to `Result`. The main thing to take away here is that Rust’s union types interoperate correctly with C’s unions as long as you mark it as `#[repr(C)]` at the Rust type definition.

#### Enums

The *other* reason you might care about union types is that Rust enums interoperate with C using unions—because C’s enum types aren’t up to the task, since they’re essentially just names for integer values. If you put `[repr(C)]` on a Rust enum, it will be laid out as a <i>union</i>, with a Rust enum defining the tags, and structs for each variant. You’ll still need to do the work of defining these types on the C side, but this way you can impose some type-safety on enum interop. There are some more important details in [RFC #2195], which defined all of this, so I encourage you to read it!

[RFC #2195]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md

#### Tuples

C also has no notion of a *tuple*, so languages consuming Rust via C APIs can’t interact with Rust tuples—even if the language in question *does* have its own notion of tuples. You can work around this by using a full-on `struct` to pass data back and forth, with conversions between your tuples and that struct type. The distinction between a <i>tuple</i> and a <i>tuple struct</i> is important here: tuples are anonymous types, while tuple structs have type names, which lets us write corresponding types in C for them, using index-style names for the fields: `_0`, `_1`, etc.

## Outro

There is of course far more that could be said about this, but that should be enough to get you started! And next week, I’ll be back with a Crates You Should Know episode diving into a bunch of crates that make all of this *way* easier.

### Patreon Sponsors

Thanks as always to this month’s $10-or-more sponsors:

- Arun Kulshreshtha
- Matt Rudder
- Soren Bramer Schmidt
- Dan Abrams
- Oluseyi Sonaiya
- Anthony Deschamps
- Evan Stoll
- Nick Gideo
- Dominic Cooney
- Embark Studios
- Scott Moeller
- Benjamin Manns
- Daniel Mason
- Jonathan Knapp
- Nick Stevens
- Jeff May
- Behnam Esfahbod
- Johan Andersson
- Nathan Sculli
- James Hagans II
- John Rudnick
- Zach Peters
- Chip
- Jerome Froelich
- Andrew Dirksen
- Joseph Schrag
- Brian McCallister
- Bryan Stitt
- Raph Levien
- Nicolas Pochet
- Daniel Bornkessel (April only)
- Ryan Osial
- Jason Bowen
- Jako Danar
- Michael Mc Donnell
- Adam Green
- Alexander Payne
- Rob Tsuk
- David Carroll
- Ramon Buckland
- Martin Heuschober
- Peter Tillemans
- Paul Naranja
- Graham Wihlidal
- Ola Fadeyi
- Cristian Paul
- Daniel Collin

### Show info

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/cysk/bindgen/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!