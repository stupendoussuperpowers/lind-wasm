#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#define LOOP_COUNT 1000000

long long gettimens() {
	struct timespec tp;
	clock_gettime(CLOCK_MONOTONIC, &tp);
	return (long long)tp.tv_sec * 1000000000LL + tp.tv_nsec;
}

void file_exists() {
	long long start_time = gettimens();

	for (int i = 0; i < LOOP_COUNT; i++) {
		int fd = open("file-read.txt", O_RDONLY, 0);
		close(fd);
	}

	long long end_time = gettimens();
	long long total_time = end_time - start_time;

	long long avg_time = total_time / LOOP_COUNT;
	fprintf(stderr, "File Exists - Loops: %d | Avg Time: %lld ns\n",
		LOOP_COUNT, avg_time);
	fflush(stderr);
}

void file_ocreat() {
	long long start_time = gettimens();

	for (int i = 0; i < LOOP_COUNT; i++) {
		int fd = open("non-exists", O_CREAT, 0);
		close(fd);
	}

	long long end_time = gettimens();
	long long total_time = end_time - start_time;

	long long avg_time = total_time / LOOP_COUNT;
	fprintf(stderr, "File Exists - Loops: %d | Avg Time: %lld ns\n",
		LOOP_COUNT, avg_time);
	fflush(stderr);
}

int main(int argc, char *argv[]) {
	file_exists();

	// file_ocreat();
}
