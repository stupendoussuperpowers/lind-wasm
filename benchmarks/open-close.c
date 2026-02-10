#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#define LOOP_COUNT 1024

long long gettimens() {
	struct timespec tp;
	clock_gettime(CLOCK_MONOTONIC, &tp);
	return (long long)tp.tv_sec * 1000000000LL + tp.tv_nsec;
}

int main(int argc, char *argv[]) {
	int fds[LOOP_COUNT];

	for (int i = 0; i < LOOP_COUNT; i++) {
		fds[i] = open(".", O_RDONLY, 0);
	}

	long long start_time = gettimens();

	for (int i = 0; i < LOOP_COUNT; i++) {
		close(fds[i]);
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
