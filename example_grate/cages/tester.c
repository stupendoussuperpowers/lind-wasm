#include <unistd.h>
#include <sys/stat.h>
#include <sys/utsname.h>
#include <fcntl.h>
#include <stdio.h>
#include <error.h>
#include <stdlib.h>

int main() {
	printf("\n[PROG]: Top\n");
        int fd = open("etest.wasm", O_WRONLY, 0755);
        printf("\n[PROG]: FD: %d\n", fd);

	close(fd);
	return 0;
}
