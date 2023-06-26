#define ARRAY_LENGTH 10
#include <stddef.h>
#include <stdint.h>

void daxpy_1_1(int64_t n, double da, double *dx, double *dy);
// void daxpy_1_1_sve(int64_t n, double da, double *dx, double *dy);
void print_array(size_t len, double arr[len]);
