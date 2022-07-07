/*
 * whotofile.c: show how to redirect output for another program
 * 
 * action: fork, and then redirect in the child process, then exec
*/

#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <assert.h>

/*
 * purpose: redirect fd(0/1/2) to the file specified in `file_name`
 * 
 * action: use open-dup2-close approach
*/
void redirect(int new_fd, char *file_name)
{
	int old_fd = open(file_name, O_CREAT | O_RDWR | O_TRUNC, 0644);
	assert(old_fd != -1);

	assert(new_fd == dup2(old_fd, new_fd));
	assert(close(old_fd) != -1);
}

int main()
{
	int pid = -1;
	if (-1 == (pid = fork())) {
		perror("fork");
		exit(1);
	} else if (pid == 0) {
		// in the child process
		redirect(1, "userlist");
		if (-1 == execlp("who", "who", NULL)) {
			perror("execlp");
			exit(1);
		}
	}

	return 0;
}
