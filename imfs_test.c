#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>

int main() {
	printf("Opening\n");	
	int fd = open("tmpfile.txt", O_CREAT | O_WRONLY, 0755);
	printf("fd=%d\n", fd);
	if(fd < 0) {
		printf("Open error\n");
		perror("open error");
	}

	printf("Writing\n");	
	int wt = write(fd, "hello", 5);
	if(wt < 0) {
	 	printf("Write error\n");
	 	perror("write error");
	}
	
	printf("Opening 2\n");	
	int fd2 = open("TEST_NODE", O_RDONLY, 0);
	printf("fd2=%d\n", fd2);
	if(fd2 < 0) {
		printf("open 2 error\n");
		perror("open 2 error");
	}
	
	printf("Closing\n");	
	int cls = close(fd);
	if(cls < 0) {
		printf("close error\n");
		perror("close error");
	}

	char *buf = malloc(256);

	printf("Reading\n");	
	int rd = read(fd2, buf, 10);
	if(rd < 0) {
		printf("read error rd=%d errno=%d\n", rd, errno);
		perror("read error");
	}
	printf("File=%s Data=%s\n", "TEST_NODE", buf);
	
	int fd3 = open("tmpfile.txt", O_RDONLY, 0);
	if(fd3 < 0) {
		printf("open 3 error fd3=%d\n", fd3);
		perror("open 3 error");
	}

	rd = read(fd3, buf, 5);
	if(rd < 0) {
		printf("read 2 error\n");
		perror("read 2 error");
	}
	printf("File=%s Data=%s\n", "tmpfile.txt", buf);
	
	close(fd2);
	
	return 0;
}
