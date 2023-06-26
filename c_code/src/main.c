#include "test_sve.h"

#define ARRAY_LENGTH 10

int main()
{
	double a = 1;
	double b[ARRAY_LENGTH] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
	double c[ARRAY_LENGTH] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};

	daxpy_1_1(10, a, b, c);

	print_array(ARRAY_LENGTH, b);
	print_array(ARRAY_LENGTH, c);

	double d[ARRAY_LENGTH] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
	double e[ARRAY_LENGTH] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
	daxpy_1_1_sve(10, a, d, e);

	print_array(ARRAY_LENGTH, d);
	print_array(ARRAY_LENGTH, e);

	// TEST SHIFT LEFT
	uint64_t x[12] = {0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048};
	uint64_t y[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
	uint64_t result[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	sve_shift_left(x, y, result);
	print_uint_array(12, result);

	// TEST SHIFT RIGHT
	uint64_t x_1[12] = {0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048};
	uint64_t y_1[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
	uint64_t result_1[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	sve_shift_right(x_1, y_1, result_1);
	print_uint_array(12, result_1);

	// TEST ADD
	uint64_t x_2[12] = {UINT64_MAX, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
	uint64_t y_2[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
	uint64_t result_2[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	uint64_t overflowed[12] = {2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2};
	sve_add(x_2, y_2, result_2, overflowed);
	print_uint_array(12, result_2);
	print_uint_array(12, overflowed);

	// TEST SUBSTRACT
	uint64_t x_3[12] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
	uint64_t y_3[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
	uint64_t result_3[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	sve_substract(x_3, y_3, result_3);
	print_uint_array(12, result_3);

	// TEST WILL OVERFLOW
	uint64_t x_4[12] = {UINT64_MAX, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
	uint64_t y_4[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
	uint64_t result_4[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	sve_will_sum_overflow(x_4, y_4, result_4);
	print_uint_array(12, result_4);

	return 0;
}
