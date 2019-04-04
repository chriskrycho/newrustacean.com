#include "ffi-demo.h"

int add(int a, int b) {
    return a + b;
}

void translate(Point * point, float byX, float byY) {
    point->x += byX;
    point->y += byY;
}
