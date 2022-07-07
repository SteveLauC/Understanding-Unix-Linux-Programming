/*
 * hello_single.c: a single threaded hello world program
*/

#include <stdio.h>
#include <unistd.h>
#define NUM 5

void print_msg(char *);

int main(void)
{
	print_msg("hello");
	print_msg("world\n");
}

void print_msg(char *msg)
{
	for (int i = 0; i < NUM; i += 1) {
		printf("%s", msg);
		fflush(stdout);
		sleep(1);
	}
}
