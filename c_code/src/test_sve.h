#include <stddef.h>
#include <stdint.h>
#ifdef __ARM_FEATURE_SVE
#include <arm_sve.h>
#endif /* __ARM_FEATURE_SVE */

#define ARRAY_LENGTH 10
#define STATE_WIDTH 12

void daxpy_1_1(int64_t n, double da, double *dx, double *dy);
void daxpy_1_1_sve(int64_t n, double da, double *dx, double *dy);
void print_array(size_t len, double arr[len]);
void print_uint_array(size_t len, uint64_t arr[len]);
void sve_shift_left(uint64_t x[STATE_WIDTH], uint64_t y[STATE_WIDTH], uint64_t *result);
void sve_shift_right(uint64_t x[STATE_WIDTH], uint64_t y[STATE_WIDTH], uint64_t *result);
void sve_add(uint64_t x[STATE_WIDTH], uint64_t y[STATE_WIDTH], uint64_t *result, uint64_t *overflowed);
void sve_substract(uint64_t x[STATE_WIDTH], uint64_t y[STATE_WIDTH], uint64_t *result);
