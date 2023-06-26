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

	uint64_t x[12] = {0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048};
	uint64_t y[12] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1};
	uint64_t result[12] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
	shift_left_test(x, y, result);

	print_uint_array(12, result);

	return 0;
}
