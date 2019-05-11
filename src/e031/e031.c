#include <stdbool.h>
#include <stdio.h>

#include "e031.h"

void section(bool first) { printf("\n---\n\n"); }

int main() {
    // --- Start easy: just numbers. --- //
    section(true);
    int a = 123;
    int b = 456;
    printf("Added `%d` to `%d` in Rust, and got back `%d`.\n", a, b,
           add_in_rust(123, 456));

    // --- Strings are a mite more interesting. --- //
    section(false);

    // We start by allocating a couple strings on the C side.
    char *greeting = "Hello, ";
    char *name = "Rustacean!";

    // Then we get the result of concatenating them in Rust, and we get back a
    // pointer to memory that Rust owns.
    char *joined = concat_strings(greeting, name);
    printf("A greeting: `%s`\n", joined);

    // We always need to free memory when we're done; otherwise we create a
    // memory leak. However, because Rust owns this memory, Rust needs to free
    // it! We effectively have an unsafe mutable reference to memory which Rust
    // is keeping alive -- returning it this way lets Rust drop it.
    free_rust_string(joined);

    // --- What about structs? --- //
    section(false);

    // Start by allocating a point.
    point_t point = {.x = 0.4, .y = 05};
    printf("`point` starts at `%.1f, %.1f`.\n", point.x, point.y);

    // Then transpose it, using the Rust function.
    float byX = 4.8;
    float byY = 5.9;
    point_transpose(&point, byX, byY);
    printf("After transposing `point.x` by `%.1f` and `point.y` by `%.1f`, "
           "`point` is at `%.1f, %.1f`.\n",
           byX, byY, point.x, point.y);

    // But because we have access to the internals on the C side, we can also
    // mutate it here. This is *okay*, but it means Rust has no control, and we
    // have to keep track of who has done what to it. And shared mutable state
    // is the root of all evil; this is *not* what we want.
    point.x = 43.4;
    point.y = 55.5;
    printf("...and we just mutated it on the C side to `%.1f, %.1f`.\n",
           point.x, point.y);

    // Again, we leak memory if we don't call the Rust `free`.
    point_free(&point);

    // --- Now, for opaque pointers! --- //
    section(false);

    // Here, we are getting an `OpaquePoint` from Rust, and the *only* way we
    // can construct such a point is by using the means Rust supplies. This is
    // helpful because it means we can actually maintain the type's internal
    // variants however it needs to, and can use privacy effectively as
    // discussed in e030.
    opaque_point_t *opaquePoint = opaque_point_new(-4.5, 2.5);

    // Similarly, we can only *operate* on it using Rust-exposed functions. We
    // don't have access to its internal layout or even know anything about it,
    // so if we want a description, we need to let Rust supply it. Note that, as
    // in the string example above, we're allocating a string here and we need
    // to be sure to call free on it! If we call `opaque_point_describe` in an
    // inline form --
    //
    //     printf("`opaquePoint` starts at %s.\n",
    //            opaque_point_describe(opaquePoint));
    //
    // -- we would end up with a memory leak, because we would have no way to
    // free the reference returned by Rust.
    char *description;
    description = opaque_point_describe(opaquePoint);
    printf("`opaquePoint` starts at %s.\n", description);
    free_rust_string(description);

    // Unlike with `Point`, we *cannot* simply mutate this one on the C side (or
    // in any other language consuming it via C FFI). Instead, we must used the
    // `opaque_point_transpose` function to do it. This works identically to how
    // `point_transpose` works; it's just that this is the *only* way to do it.
    float opaqueByX = 8.8;
    float opaqueByY = -1.0;
    opaque_point_transpose(opaquePoint, opaqueByX, opaqueByY);

    // Once again we're allocating a string on the Rust side we'll need to free.
    description = opaque_point_describe(opaquePoint);
    printf("After transposing `opaquePoint.x` by `%.1f` and `opaquePoint.y` by "
           "`%.1f`, `opaquePoint` is at %s.\n",
           opaqueByX, opaqueByY, description);

    // And here we do just that.
    free_rust_string(description);

    // And we also need to free `opaquePoint` so we don't leak *its* memory.
    opaque_point_free(opaquePoint);
}
