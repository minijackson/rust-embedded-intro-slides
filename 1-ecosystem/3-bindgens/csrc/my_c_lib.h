#pragma once

#include <stdint.h>

typedef struct {
	uint8_t radius;
	char const* name;
} circle_t;

void print_uint8(uint8_t value);

typedef int64_t(*callback_t)(void);
void print_callback_result(callback_t callback);

void print_circle(circle_t circle);

void call_rust_from_c(void);
