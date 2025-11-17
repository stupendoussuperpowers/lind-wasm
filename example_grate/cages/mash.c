#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <sys/wait.h>

#include <fcntl.h>

#include <dirent.h>

static char *trim(char *s) {
	while (isspace(*s))
		s++;
	if (*s == 0)
		return s;
	char *end = s + strlen(s) - 1;
	while (end > s && isspace(*end))
		end--;
	end[1] = '\0';
	return s;
}

// split a string like "cmd arg1 arg2" into argv[]
static char **make_argv(char *cmd) {
	int cap = 8, n = 0;
	char **argv = malloc(sizeof(char *) * cap);
	char *tok = strtok(cmd, " \t");
	while (tok) {
		if (n + 1 >= cap) {
			cap *= 2;
			argv = realloc(argv, sizeof(char *) * cap);
		}
		argv[n++] = tok;
		tok = strtok(NULL, " \t");
	}
	argv[n] = NULL;
	return argv;
}

void list_dir(const char *path) {
	DIR *d = opendir(path);
	struct dirent *entry;
	printf("Contents of %s:\n", path);
	while ((entry = readdir(d)) != NULL) {
		printf("  %s\n", entry->d_name);
	}
	closedir(d);
	printf("\n");
}

int main(int argc, char **argv) {
	if (argc < 3 || strcmp(argv[1], "-c") != 0) {
		fprintf(stderr, "usage: %s -c \"command\"\n", argv[0]);
		return 1;
	}

	int fd = open("etest.wasm", O_WRONLY, 0);
	printf("FD etest: %d\n", fd);
	close(fd);

	int nchildren = 0;

	char *cmd = strdup(argv[2]); // full string
	char *saveptr;
	char *segment = strtok_r(cmd, "&", &saveptr);

	while (segment) {
		char *t = trim(segment);
		if (*t) {
			pid_t pid = fork();
			if (pid == 0) {
				// child: exec command
				char *copy = strdup(t);
				char **cmd_argv = make_argv(copy);

				execv(cmd_argv[0], cmd_argv);
				char errbuf[256];
				snprintf(errbuf, sizeof(errbuf), "execv(%s)",
					 cmd_argv[0]);
				perror(errbuf);
				exit(1);
			} else if (pid < 0) {
				perror("fork");
			} else {
				nchildren++;
			}
			// parent: do not wait (background)
		}
		segment = strtok_r(NULL, "&", &saveptr);
	}

	// behave like bash when all jobs are background:
	// exit immediately, but reap zombies
	for (int i = 0; i < nchildren; i++) {
		int status;
		pid_t w = wait(&status);
		if (w > 0) {
			printf("[mash] terminated pid %d, status %d\n", w,
			       status);
		}
	}

	return 0;
}
