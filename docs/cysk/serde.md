# Crates You Should Know: Serde

Hello, I’m Chris Krycho, and this is New Rustacean: a show about the Rust programming language and the people who use it. This is **Crates You Should Know: Serde.**

## Intro

It’s actually kind of amazing to me that we’re two and a half years into the show before I finally get around to talking about Serde. It’s been mentioned on the show repeatedly, and it’s one of the absolute core building blocks of the Rust ecosystem, and it’s frankly just a really impressive piece of software in its own right – inside the Rust ecosystem *or* outside. So now we’re going to do a solid dive into the library’s purpose, approach, and (at a high level) APIs!

## Serde

Serde is a library for *serialization* and *deserialization* of data – data of all sorts. The name is just a mashup of the first syllable of *serialization* and *deserialization* - ‘s’ ‘e’ ‘r’ from serialization and ‘d’ ‘e’ from deserialization.

One thing that distinguishes Serde from e.g. the JSON-to-data-structure implementation in your other favorite language is that *most* of the time, the serialization and deserialization that happens, and really the data layer transformations of any sort (including for ORMs) are done at *runtime*, by mapping over the supplied data structure. Serde and Rust obviously still have to do some work at runtime, but that work is a lot narrower in scope, because they use Rust’s types and Serde’s traits to avoid any runtime attribute tagging or reflection. As is often the case with this kind of trait-driven programming, the compiler can often optimize it down to the same kind of performance you’d get by hand-writing a serializer... but as we’ll cover in a moment, you pretty much never have to hand-write serializers!

The current list of supported formats for serialization and deserialization is pretty long! Among others, you can serialize and/or deserialize to JSON, YAML, MessagePack, TOML, Pickle (from Python), URL, and XML. (And there are more!)

### Using Serde as a consumer

Serde, as you might expect, leans heavily on a few core traits: `Serialize` and `Deserialize`. And, happily, these are traits you’ll only *very* rarely need to implement yourself. Courtesy of Rust’s support for *custom derives*—which Sean Griffin and I talked about a bit in [our discussion of Diesel 1.0](https://www.newrustacean.com/show_notes/interview/diesel_1_0/part_1/index.html "Diesel 1.0, Part 1"). For Serde, you can take *essentially* any data type you can define in Rust and simply use the `derive` attribute by writing `#[derive(Serialize, Deserializer)]` in front of any type you define. In a few rare occasions, you’ll need to write the implementations yourself (and the really excellent docs have you covered in that case), but usually you’ll just derive these traits automatically.

To make that work, you’ll need to make a couple tweaks to your project besides adding Serde itself to the dependency list.

1. Add `serde_derive` to your Cargo.toml. It’s an *optional* dependency of Serde—and it’s probably safe to say it’s *usually* installed alongside it! But if you happen to be in a scenario where you’re not going to be using the derives, you don’t have to install it.

2. Add the dependencies for the target format you want to use—again, Serde breaks out its support for each format into its own crate. This keeps your build size down: there’s no reason to include, say, `serde_yaml` if you’re only targeting JSON.

3. In your `main.rs` or `lib.rs` file—depending on whether you’re building an application or a library—add the top-level attribute to include the custom derive macro: `#[macro_use] extern crate serde_derive`.

And that’s it! You can now derive `Serialize`, `Deserialize`, or both on any `struct` or `enum` you need to serializer or deserialize.

Once you’ve set up a data type with the relevant attributes, you can serialize and deserialize them by using the main functions a Serde target library supplies. The most common ones are:

- `ser::to_string`: the function for serializing a given Rust item into a string of a given format
- `de::from_string`: the function for deserializing a string of a given format into a Rust item

For other target formats, you’d see different names—if the Serde library you were using targeted binary buffers, for example, it would be `ser::to_bytes` and `de::from_bytes`.

Once you have the basic configuration all set up, you can start *using* those functions with your types.

You can also further customize the way a data type is serialized and deserialized by using other *attributes*. Serde has attributes for customizing the container (the struct or enum itself); customizing enum variants; and customizing struct fields. For example, you can rename a struct field or enum variant. These kinds of things might come in handy when you’re translating between the expectations from different data layers in other languages, for example. In JavaScript, it’s normal for payloads to be formatted in `camelCase`—lowercase first letter, capitalized first letter of all words following in a name—but since it’s normal for Rust fields to be `snake_case`—all lowercase, separating words in a name with underscores—you can use Serde’s `rename_all` container attribute. You can also skip fields and variants entirely, or define custom functions to serialize and deserialize them.

In short, Serde has *really* good defaults but also the flexibility to do most of what you need via attributes if the defaults don’t match your specific scenario... and the ability to just write your own specific `Serialize` and `Deserialize` implementation yourself if even *those* don’t get you far enough.

### Using Serde as a producer

The other use case for Serde is creating support for a new source for deserialization and target for serialization. As I noted earlier, Serde already supports a wide array of target formats. However, there are cases where you might need to write your own format for a given data representation type, or for a given protocol for transmission. In that case, you can implement your own serializer and deserializer plugin library, just like the existing libraries like `serde_json`.

Serde—as its docs emphasize!—is not a *parsing* library. Its functionality is *specifically* for converting data to or from a given, *already-parsed* format and Rust data structures. So, for example, `serde_json` [parses the JSON itself](https://github.com/serde-rs/json/blob/master/src/de.rs#L110), iterating through a buffer of bytes, and likewise actually [generates the JSON itself](https://github.com/serde-rs/json/blob/master/src/ser.rs#L72) (including supplying a variety of options for how you print the JSON).

The API you have to implement is not *large*, and it is *well-documented*. For any given data type, you have to implement the `Serialize` and/or `Deserialize` traits (depending on what you’re doing), and conventionally you should also supply `to_` and/or `from_` functions, like `to_str` and `from_str` for serializing and deserializing `&str` string buffers, or `to_bytes` and `from_bytes` for serializing and deserializing directly to `&[u8]` bytes buffers.

I’m not going to try to talk through everything involved in writing your own implementation; that would quickly involve far too much trying to describe code aloud, and I do enough of that in this podcast! Instead, if you’re interested in seeing a really quite straightforward and relatively easy to understand example, I recommend blocking out a couple hours and reading through the `serde_json` implementation. You’ll probably learn a lot!

## Outro

So that’s Serde! If you need to serialize or deserialize stuff in Rust… this is where it’s at!

### Sponsors

Thanks, as always, to all of this month's sponsors. Contributors who gave at least $10 included:

- Aaron Turon
- Alexander Payne
- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Hans Fjällemark
- John Rudnick
- Matt Rudder
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Olaf Leidinger
- Oluseyi Sonaiya
- Raph Levien
- and Vesa Khailavirta

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for *this* episode at \<newrustacean.com/show\_notes/cysk/serde/\>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!