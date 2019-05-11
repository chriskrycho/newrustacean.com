/// The `add_in_rust` function exported by the Rust crate.
extern int add_in_rust(int a, int b);

/// The `concat_strings` function exported by the Rust crate.
extern char *concat_strings(char const *const first, char const *const second);

/// The `free_rust_string` function exported by the Rust crate.
extern void free_rust_string(char *to_free);

/// The `Point` type declared by Rust. We're not making this opaque; see below.
typedef struct point {
    float x;
    float y;
} point_t;

/// The `point_translate` function declared by Rust. Mutates `point`!
void point_translate(point_t *point, float byX, float byY);

/// An opaque type: C cannot do anything with the internals, unlike `point_t`.
typedef struct opaque_point opaque_point_t;

/// The `opaque_point_new` function declared by Rust. Gets us a `point`!
opaque_point_t *opaque_point_new(float x, float y);

/// The `opaque_point_translate` function declared by Rust. Mutates `point`!
void opaque_point_translate(opaque_point_t *point, float byX, float byY);

/// The `opaque_point_describe` function declared by Rust. Must free the string
/// it returns!
char *opaque_point_describe(opaque_point_t *point);

/// The `opaque_point_free` function declared by Rust.
void opaque_point_free(opaque_point_t *point);
