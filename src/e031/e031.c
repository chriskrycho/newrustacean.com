#include <stdio.h>

#include "e031.h"

int main() {
    int a = 123;
    int b = 456;
    printf("Added `%d` to `%d` in Rust, and got back `%d`", a, b,
           add_in_rust(123, 456));
}
