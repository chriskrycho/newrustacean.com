#include <stdbool.h>
#include <stdio.h>

#include "e031.h"

void section(bool first) { printf("\n---\n\n"); }

int main() {
    section(true);
    int a = 123;
    int b = 456;
    printf("Added `%d` to `%d` in Rust, and got back `%d`.\n", a, b,
           add_in_rust(123, 456));

    section(false);
    char *greeting = "Hello, ";
    char *name = "Rustacean!";
    char *joined = concat_strings(greeting, name);
    printf("A greeting: `%s`\n", joined);
    // We leak memory if we don't do this!
    free_rust_string(joined);

    section(false);
    point_t point = {.x = 0.4, .y = 05};
    printf("`point` starts at `%.1f, %.1f`.\n", point.x, point.y);
    float byX = 4.8;
    float byY = 5.9;
    point_transpose(&point, byX, byY);
    printf("After transposing `point.x` by `%.1f` and `point.y` by `%.1f`, "
           "`point` is at `%.1f, %.1f`.\n",
           byX, byY, point.x, point.y);
    point.x = 43.4;
    point.y = 55.5;
    printf("...and we just mutated it on the C side to `%.1f, %.1f`.\n",
           point.x, point.y);
    // Again, we leak memory if we don't call the Rust `free`.
    point_free(&point);

    section(false);
    opaque_point_t *opaquePoint = opaque_point_new(-4.5, 2.5);
    printf("`opaquePoint` starts at %s.\n", opaque_point_describe(opaquePoint));
    float opaqueByX = 8.8;
    float opaqueByY = -1.0;
    opaque_point_transpose(opaquePoint, opaqueByX, opaqueByY);
    char *described = opaque_point_describe(opaquePoint);
    printf("After transposing `opaquePoint.x` by `%.1f` and `opaquePoint.y` by "
           "`%.1f`, `opaquePoint` is at %s.\n",
           opaqueByX, opaqueByY, described);
    // Again, we leak memory if we don't call the Rust `free`.
    opaque_point_free(opaquePoint);
    // But we also need to free `described`! If we had called it inline, as we
    // might have been tempted to, we'd have had no reference to it, and
    // therefore would have leaked its memory!
    free_rust_string(described);
}
