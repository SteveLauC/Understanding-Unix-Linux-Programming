/*
 * popen.c: a version of UNIX popen() libaray function
 *
 *
 * FILE * popen(const char * command, const char * type);
 *
 * action: execute `command`
 *
 * arguments:
 *      `command` is a normal shell command
 *      `type` is either "r" or "w"
 *
 * return: a stream attached to the `command`, or NULL
 *
 */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// due to the conclicting type of function signature, I change the name to
// `my_popen`
FILE *my_popen(const char *command, char *type)
{
	assert(command != NULL);
	assert(type != NULL);

	// get a pipe
	int pipe_fd[2] = { -1, -1 };
	if (pipe(pipe_fd) == -1) {
		perror("Can not pipe");
		return NULL;
	}

	// fork a process
	int pid = fork();
	if (pid == -1) {
		perror("Can not fork");
		return NULL;
	} else if (pid == 0) {
		// child-process
		if (*type == 'r') {
			close(pipe_fd[0]);
			// redirect stdout to the pipe write end
			dup2(pipe_fd[1], 1);
		} else if (*type == 'w') {
			close(pipe_fd[1]);
			// redirect stdin to the pipe read end
			dup2(pipe_fd[0], 0);
		} else {
			fprintf(stderr, "invalid type argument");
			return NULL;
		}
		// execute `command`
		execl("/usr/bin/bash", "sh", "-c", command, NULL);
	} else {
		// parent-process
		if (*type == 'r') {
			close(pipe_fd[1]); // close the write end
			return fdopen(pipe_fd[0], type);
		} else if (*type == 'w') {
			close(pipe_fd[0]);
			return fdopen(pipe_fd[1], type);
		} else {
			fprintf(stderr, "invalid type argument");
			return NULL;
		}
	}
}

// use code from `popendemo.c` to test this
int main()
{
	FILE *fp = NULL;
	char buf[100] = { '\0' };
	int i = 0;

	fp = my_popen("who|sort", "r");

	while (fgets(buf, 100, fp) != NULL) {
		printf("%3d %s", i++, buf);
	}

	pclose(fp);
	return 0;
}
