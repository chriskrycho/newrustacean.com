#include "ffi-demo.h"

void translate(Point * point, float byX, float byY) {
    point->x += byX;
    point->y += byY;
}
