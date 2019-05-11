#include <stdio.h>

#include "e031.h"

int main() {
    int a = 123;
    int b = 456;
    printf("Added `%d` to `%d` in Rust, and got back `%d`.\n", a, b,
           add_in_rust(123, 456));

    char *greeting = "Hello, ";
    char *name = "Rustacean!";
    char *joined = concat_strings(greeting, name);
    printf("A greeting: `%s`\n", joined);
    // We leak memory if we don't do this!
    free_rust_string(joined);
}
