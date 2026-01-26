#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>

#define LOOP_COUNT 10000000

long long gettimens() {
    struct timespec tp;
    clock_gettime(CLOCK_MONOTONIC, &tp);
    return (long long)tp.tv_sec * 1000000000LL + tp.tv_nsec;
}

unsigned long long fibonacci(int n) {
    if (n <= 1) return n;
    volatile  unsigned long long a = 0, b = 1, c;
    for (int i = 2; i <= n; i++) {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}

int main() {
    unsigned long long sum = 0;

    long long start_time = gettimens();

    for (int i = 0; i < LOOP_COUNT; i++) {
        sum += fibonacci(1000);
    }

    long long end_time = gettimens();

    long long avg_time = (end_time - start_time) / LOOP_COUNT;
    fprintf(stderr, "Loop Count: %d | Avg Time: %lld ns\n", LOOP_COUNT, avg_time);

    return 0;
}
