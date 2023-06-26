#define ARRAY_LENGTH 10
#include <stddef.h>
#include <stdint.h>

void daxpy_1_1(int64_t n, double da, double *dx, double *dy);
void daxpy_1_1_sve(int64_t n, double da, double *dx, double *dy);
void print_array(size_t len, double arr[len]);
void print_uint_array(size_t len, uint64_t arr[len]);
void shift_left_test(uint64_t x[12], uint64_t y[12], uint64_t *result);
