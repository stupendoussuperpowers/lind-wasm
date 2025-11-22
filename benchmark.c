#include <time.h>
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
	// printf("CAGEID: %d\n", getpid());
	int loops = 1000000;

	for(int i = 0; i < loops; i++) {
		int ret = geteuid();
	}
}
