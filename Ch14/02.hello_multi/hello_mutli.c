/*
 * hello_multi.c: a multi-threaded hello world program
*/

#include <stdio.h>
#include <unistd.h>
#include <pthread.h>

#define NUM 5

void *print_msg(void *m);

int main(void)
{
	pthread_t t1;
	pthread_t t2;

	pthread_create(&t1, NULL, print_msg, "hello");
	pthread_create(&t2, NULL, print_msg, "world\n");

	pthread_join(t1, NULL);
	pthread_join(t2, NULL);
}

void *print_msg(void *m)
{
	char *cp = (char *)m;

	for (int i = 0; i < NUM; i += 1) {
		printf("%s", cp);
		fflush(stdout);
		sleep(1);
	}

	return NULL;
}
