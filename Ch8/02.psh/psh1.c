/*
    prompting shell version 1
    prompts for the command and its arguments
    builds the argument vector for the call to execvp
    Uses execvp(), and never returns.
*/

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>

#define MAXARGS 20 /* cmd line args*/
#define ARGLEN 100 /* token length*/

char *make_string(char *buf);
int execute(char *arg_list[]);

int main()
{
	int num_args = 0;
	char buf[ARGLEN + 1];
	*buf = '\0';
	char *arg_list[MAXARGS + 1];
	for (int i = 0; i < MAXARGS + 1; i++) {
		arg_list[i] = NULL;
	}

	while (num_args < MAXARGS) {
		printf("arg[%d]", num_args);

		// read user input to buf, and judge whether it is just a newline char
		if (fgets(buf, ARGLEN, stdin) && *buf != '\n') {
			arg_list[num_args++] = make_string(buf);
		} else {
			// if we have at least one arg, then execute it
			if (num_args > 0) {
				arg_list[num_args] = NULL;
				execute(arg_list);

				// reset num_args so that we "can" continuously run commands
				num_args = 0;
			}
		}
	}

	return 0;
}

/*
    use execvp to execute the cmd
*/
int execute(char *arglist[])
{
	if (-1 == execvp(arglist[0], arglist)) {
		const char *p = NULL;
		for (int i = 0; i < MAXARGS + 1; i++) {
			p = arglist[i];
			if (p != NULL) {
				fprintf(stderr, "debug: arg[%d]: %s\n", i, p);
			}
		}

		perror("execvp failed");
		exit(EXIT_FAILURE);
	}
}

/*
    trim the newline character and allocate the memory for args
*/
char *make_string(char *buf)
{
	// trim the rightmost newline char
	buf[strlen(buf) - 1] = '\0';

	// allocate the memory
	char *cp = (char *)malloc(sizeof(strlen(buf) + 1));

	if (NULL == cp) {
		fprintf(stderr, "no memory available");
		exit(EXIT_FAILURE);
	}
	strncpy(cp, buf, strlen(buf) + 1);

	return cp;
}