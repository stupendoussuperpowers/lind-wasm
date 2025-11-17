#include <unistd.h>
#include <sys/stat.h>
#include <sys/utsname.h>
#include <fcntl.h>
#include <stdio.h>
#include <error.h>
#include <stdlib.h>

int main() {
	int cageid = getpid();
	printf("\n---- RUNOPEN %d ----\n", cageid);
	int fd = open("firstfile", O_CREAT | O_WRONLY, 0755);
	printf("FD: %d\n", fd);
	close(fd);
	printf("\n--EXIT RUNOPEN---\n");
	return 0;
}
