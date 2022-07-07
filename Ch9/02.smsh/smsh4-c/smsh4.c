#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <string.h>
#include "smsh.h"
#include "varlib.h"

#define DFL_PROMPT "> "

/*
    purpose: error handling
    return: nothing
    action: output the error info to stderr, and exit the program with exit code n.
*/
void fatal(char *s1, char *s2, int n)
{
	fprintf(stderr, "Error: %s, %s\n", s1, s2);
	exit(n);
}

/*
    purpose: initialize shell
    return: nothing
    action: ignore signal SIGINT and SIGQUIT
*/
void setup()
{
	extern char **environ;
	assert(environ != NULL);
	VLenviron2table(environ);
	signal(SIGINT, SIG_IGN);
	signal(SIGQUIT, SIG_IGN);
}

int main()
{
	char *cmdline = NULL;
	char *prompt = DFL_PROMPT;
	char **arg = NULL;

	setup();
	while ((cmdline = next_cmd(prompt, stdin)) != NULL) {
		if ((arg = splitline(cmdline)) != NULL) {
			process(arg);
			freelist(arg);
		}
		free(cmdline);
	}

	return 0;
}