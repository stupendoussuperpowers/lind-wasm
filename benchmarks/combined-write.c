#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include <string.h>

#define LOOP_COUNT 1000000
#define MAX 5000

long long gettimens() {
	struct timespec tp;
	clock_gettime(CLOCK_MONOTONIC, &tp);
	return (long long)tp.tv_sec * 1000000000LL + tp.tv_nsec;
}

void write_size(size_t count) {
	// char *buf = malloc(count);
	char buf[MAX];
	memset(buf, 'A' + (count % 26), count);

	int fd = open("file-write.txt", O_CREAT | O_WRONLY, 0644);

	long long start_time = gettimens();

	for (int i = 0; i < LOOP_COUNT; i++) {
		pwrite(fd, buf, count, count);
	}

	long long end_time = gettimens();
	long long total_time = end_time - start_time;

	close(fd);

	long long avg_time = total_time / LOOP_COUNT;
	fprintf(stderr, "Write [%ld]\tLoops: %d | Avg Time: %lld ns\n", count,
		LOOP_COUNT, avg_time);

	fd = open("file-write.txt", O_RDONLY, 0);

	char buf2[MAX];
	pread(fd, buf2, count, count);

	if (memcmp(buf, buf2, count) != 0)
		fprintf(stderr, "Mismatch\n%.*s\n%.*s\n", (int)count, buf,
			(int)count, buf2);

	close(fd);

	fflush(stderr);
}

int main(int argc, char *argv[]) {
	write_size(10);
	write_size(4096);
}
