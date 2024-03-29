/*
 * twordcount2.c: threaded word counter for two files 
 *
 * version 2: uses mutex to lock counter
 */
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <ctype.h>

int total_words = 0;
pthread_mutex_t counter_lock = PTHREAD_MUTEX_INITIALIZER;

void *count_words(void *);

int main(int ac, char *av[])
{
	if (ac != 3) {
		printf("usage: ./twordcount1 file1 file2\n");
		exit(1);
	}

	pthread_t t1;
	pthread_t t2;

	pthread_create(&t1, NULL, count_words, (void *)av[1]);
	pthread_create(&t2, NULL, count_words, (void *)av[2]);

	pthread_join(t1, NULL);
	pthread_join(t2, NULL);

	printf("%d: total words\n", total_words);
	return 0;
}

void *count_words(void *f)
{
	char *file_name = (char *)f;
	FILE *fp = fopen(file_name, "r");

	if (fp == NULL) {
		perror(file_name);
	}

	int c = EOF;
	int prev = '\0';
	while ((c = getc(fp)) != EOF) {
		if (!isalnum(c) && isalnum(prev)) {
			pthread_mutex_lock(&counter_lock); // lock
			total_words += 1;
			pthread_mutex_unlock(&counter_lock); // unlock
		}
		prev = c;
	}

	fclose(fp);
	return NULL;
}
