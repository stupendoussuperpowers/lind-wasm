#include <time.h>
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
	int loops = 1000000;

	for(int i = 0; i < loops; i++) {
		int ret = geteuid();
	}
}
