#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <stdlib.h>
#include <unistd.h>
#include <register_handler.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <cp_data_between_cages.h>

#include <imfs.h>

// Function ptr and signatures of this grate
typedef int (*func_ptr_t)(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

int open_grate(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

int close_grate(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

int read_grate(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

int write_grate(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

func_ptr_t func_array[4] = { open_grate, close_grate, read_grate, write_grate };

// Dispatcher function

int pass_fptr_to_wt(uint64_t index, uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage) {
    // Only one function in this grate, so the index should only be 0
    printf("\n Passing... idx=%d cageid=%d\n\n", index, cageid);
	
    if (index > 3) {
        fprintf(stderr, "Invalid index: %llu\n", index);
        return -1; 
    }
    
    return func_array[index](cageid, arg1, arg1cage, arg2, arg2cage, arg3, arg3cage, arg4, arg4cage, arg5, arg5cage, arg6, arg6cage);
}


int open_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage){
	fprintf(stderr, "[ENTER] open arg1=%s arg2=%d arg3=%d\n", arg1, arg2, arg3);
	int thiscage = getpid();
	char *pathname = malloc(256);
	if(pathname == NULL){
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	cp_data_between_cages(thiscage, arg1cage, arg1, arg1cage, (uint64_t)pathname, thiscage, 256, 1);

	int ifd  = imfs_open(cageid, pathname, arg2, arg3); 

	if(ifd < 0) {
		fprintf(stderr,"\nerrno=%d\n\n", errno);
		perror("\n\nimfs open failed.");
	}

	return ifd;
}


int close_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage){
	
	fprintf(stderr,"[ENTER] close arg1=%d arg2=%d\n", arg1, arg2);
	int thiscage = getpid();
	
	int ret =  imfs_close(cageid, arg1);
	fprintf(stderr,"[ENTER] close thiscage=%d cageid=%d ret=%d\n\n",thiscage, cageid, ret );
	return ret;
}

int read_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage){
	
	fprintf(stderr,"[ENTER] read arg1=%d arg2=%d arg3=%d\n\n", arg1, arg2, arg3);
	int thiscage = getpid();
        fprintf(stderr,"[ENTER] cageid=%d thiscage=%d\n\n", cageid, thiscage);	
	int fd = arg1; 
	int count = arg3; // = 5; // 

	char *buf = malloc(256);

	if(buf == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}
	
	fprintf(stderr,"[read_grate] read fd=%d buf=%p count=%d\n\n", arg1, buf, count);
	int ret = imfs_read(cageid, arg1, buf, count);
	fprintf(stderr,"[read_grate] read ret=%d buf=%s\n\n", ret, buf);
	if (ret < 0) {
		printf("\nread error=%d errno=%d\n\n", ret, errno);
	}

	cp_data_between_cages(arg2cage,thiscage, (uint64_t) buf, thiscage, arg2, arg2cage, count, 1);

	return ret;
}

int stdout_cage[2];
int stderr_cage[2];

static int write_host(uint64_t fd, const char *buf, uint64_t count) {
	fprintf(stderr, "Host write to: %d", fd);

	FILE *out;

	switch(fd) {
		case 1: 
			out = fdopen(stdout_cage[1],"w");
			break;
		case 2:
			out = fdopen(stderr_cage[1], "w");
			break;
		default:
			return 0;
	}
	
	// fprintf(out, "%s\0", buf);
	// fflush(out);
	fwrite(buf, 1, count, out);
	fprintf(stderr, "Written: %s\0", buf);
	fclose(out);
	return count;
}

int write_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage){
	
	fprintf(stderr,"[ENTER] write arg1=%d, arg2=%d, arg3=%d\n", arg1, arg2, arg3);
	int thiscage = getpid();
	char *buffer = malloc(256);
	

	if(buffer == NULL){
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	cp_data_between_cages(thiscage, arg2cage, arg2, arg2cage, (uint64_t)buffer, thiscage, 256, 1);

	if(arg1 < 3) {
		return write_host(arg1, buffer, arg3);
	}

	int ifd  = imfs_write(cageid, arg1, buffer, arg3); 

	if(ifd < 0) {
		fprintf(stderr,"\nwrite errno=%d\n\n", errno);
	}

	return ifd;
}

// Main function will always be same in all grates
int main(int argc, char *argv[]) {

    imfs_init();
    fprintf(stderr, "\nimfs_init() called.\n\n");	
    // Should be at least two inputs (at least one grate file and one cage file)
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <cage_file> <grate_file> <cage_file> [...]\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    int grateid = getpid();

    // Because we assume that all cages are unaware of the existence of grate, cages will not handle the logic of `exec`ing 
    // grate, so we need to handle these two situations separately in grate. 
    // grate needs to fork in two situations: 
    // - the first is to fork and use its own cage; 
    // - the second is when there is still at least one grate in the subsequent command line input. 
    // In the second case, we fork & exec the new grate and let the new grate handle the subsequent process.
    
    pipe(stdout_cage);
    pipe(stderr_cage);

    for (int i = 1; i < (argc < 3 ? argc : 3); i++) {
        pid_t pid = fork();
	fprintf(stderr, "i=%d\n", i);
        if (pid < 0) {
            perror("fork failed");
            exit(EXIT_FAILURE);
        } else if (pid == 0) {
            if (i % 2 != 0) {
                int cageid = getpid();
		int ret;
	
		dup2(stdout_cage[1], STDOUT_FILENO);
		dup2(stderr_cage[1], STDERR_FILENO);

		close(stdout_cage[1]);
		close(stderr_cage[1]);
	
		fprintf(stderr,"Starting Registrations\n");
		
		ret = register_handler(cageid, 10, 0, grateid);
                fprintf(stderr,"open register ret=%d\n", ret);
		
		ret = register_handler(cageid, 11, 1, grateid);
		fprintf(stderr,"close register ret=%d\n", ret);

		ret = register_handler(cageid, 12, 2, grateid);
		fprintf(stderr,"read register ret=%d\n", ret);
	
		ret = register_handler(cageid, 13, 3, grateid);
		fprintf(stderr, "write register ret=%d\n", ret);	
	    }
	    
	    fprintf(stderr,"Execing argv[i]=%s\n", argv[i]);
	    if ( execv(argv[i], &argv[i]) == -1) {
                perror("execv failed");
                exit(EXIT_FAILURE);
            }
  	    //int fd = open("randomfile.txt", 2, 2);
	    //printf("fd=%d\n", fd);
        }
    }

    int status;
    while (wait(&status) > 0) {
        printf("[Grate | geteuid] terminated, status: %d\n", status);
    }
    
    return 0;
}
