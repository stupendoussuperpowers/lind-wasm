#include <unistd.h>
#include <sys/stat.h>
#include <sys/utsname.h>
#include <fcntl.h>
#include <stdio.h>
#include <error.h>
#include <stdlib.h>

int main() {
        printf("\n[PROG]: Top\n");
        int fd = open("firstfile", O_CREAT | O_WRONLY, 0755);
        printf("\n[PROG]: FD: %d\n", fd);

	char *hello = "hello world\n";
	write(fd, hello, 11);

	close(fd);

        struct stat *st  = malloc(sizeof(struct stat));

        int statret = stat("firstfile", st);
        printf("\n[PROG] stat: %d\n", statret);

        printf("\n[PROG]: UID: %d  GID: %d Index: %d Size: %d\n", st->st_uid, st->st_gid,
			st->st_ino, st->st_size);
	
	fd = open("firstfile", O_RDONLY, 0);
	printf("\n[PROG]: FD: %d\n", fd);

	char world[11];
	read(fd, world, 11);
	printf("\n[PROG]: Read: %s\n", world);

	return 0;
}
