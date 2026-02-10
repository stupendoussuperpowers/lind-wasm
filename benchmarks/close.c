#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#define LOOP_COUNT 10000

long long gettimens() {
	struct timespec tp;
	clock_gettime(CLOCK_MONOTONIC, &tp);
	return (long long)tp.tv_sec * 1000000000LL + tp.tv_nsec;
}

int main(int argc, char *argv[]) {
	long long start_time = gettimens();

	for (int i = 0; i < LOOP_COUNT; i++) {
		close(-1);
	}

	// Get sum of time
	long long end_time = gettimens();
	long long total_time = end_time - start_time;
	// Average
	long long average_time = total_time / LOOP_COUNT;
	fprintf(stderr, "Loop Count: %d | Avg Time: %lld ns\n", LOOP_COUNT,
		average_time);
	fflush(NULL);
}
