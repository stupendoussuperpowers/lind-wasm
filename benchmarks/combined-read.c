#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#define LOOP_COUNT 1000000
#define MAX 5000

long long gettimens() {
	struct timespec tp;
	clock_gettime(CLOCK_MONOTONIC, &tp);
	return (long long)tp.tv_sec * 1000000000LL + tp.tv_nsec;
}

void read_size(size_t count) {
	char buf[MAX];

	int fd = open("file-read.txt", O_RDONLY, 0);

	long long start_time = gettimens();
	for (int i = 0; i < LOOP_COUNT; i++) {
		pread(fd, buf, count, 0);
	}
	long long end_time = gettimens();

	long long avg_time = (end_time - start_time) / LOOP_COUNT;
	fprintf(stderr, "Read [%ld]\tLoops: %d | Avg Time: %lld ns\n", count,
		LOOP_COUNT, avg_time);

	close(fd);
	fflush(stderr);
}

int main(int argc, char *argv[]) {
	read_size(10);
	read_size(4096);
}
