#include "my_c_lib.h"

#include <stdio.h>

// Rust bindings
#include "bindings.h"

void print_uint8(uint8_t value) {
	printf("From C: %d\n", value);
}

void print_callback_result(callback_t callback) {
	printf("From C with callback: %ld\n", callback());
}

void print_circle(circle_t circle) {
	printf("From C circle with radius: %d and name: '%s'\n", circle.radius, circle.name);
}

void call_rust_from_c(void) {
	say_hello();

	Point2D point = { .x = 42, .y = 69 };
	print_point(&point);

	Vec_i32* vec = make_vec(5, 42);
	print_vec(vec);
	free_vec(vec);
}
