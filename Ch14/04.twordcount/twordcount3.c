/*
 * twordcount3.c: threaded word counter for two files
 *
 * version3: one counter per file
*/

#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <ctype.h>

struct arg_set {
	char *file_name;
	int count;
};

void *count_words(void *);

int main(int ac, char *av[])
{
	if (ac != 3) {
		printf("usage: ./twordcount3 file1 file2\n");
		exit(1);
	}

	struct arg_set arg1;
	arg1.file_name = av[1];
	arg1.count = 0;
	struct arg_set arg2;
	arg2.file_name = av[2];
	arg2.count = 0;

	pthread_t t1;
	pthread_t t2;
	pthread_create(&t1, NULL, count_words, (void *)&arg1);
	pthread_create(&t2, NULL, count_words, (void *)&arg2);

	pthread_join(t1, NULL);
	pthread_join(t2, NULL);

	printf("%d: %s\n", arg1.count, arg1.file_name);
	printf("%d: %s\n", arg2.count, arg2.file_name);
	printf("%d: total words", arg1.count + arg2.count);
}

void *count_words(void *arg)
{
	struct arg_set *arg_p = (struct arg_set *)arg;

	FILE *fp = fopen(arg_p->file_name, "r");

	if (fp == NULL) {
		perror(arg_p->file_name);
	}

	int c = EOF;
	int prev = '\0';
	while ((c = getc(fp)) != EOF) {
		if (!isalnum(c) && isalnum(prev)) {
			arg_p->count += 1;
		}
		prev = c;
	}

	fclose(fp);
	return NULL;
}
