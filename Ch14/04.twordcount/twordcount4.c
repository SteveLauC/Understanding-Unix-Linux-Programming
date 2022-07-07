/*
 * twordcount4.c: threaded word counter for two files
 *
 * version4: condition variable allows counter functions to report results early
*/

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <pthread.h>
#include <ctype.h>

struct arg_set {
	char *file_name;
	int count;
};

struct arg_set *mail_box;
pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t flag = PTHREAD_COND_INITIALIZER;

void *count_words(void *);

int main(int ac, char *av[])
{
	if (ac != 3) {
		printf("usage: ./twordcount4 file1 file2\n");
		exit(1);
	}

	pthread_t t1;
	pthread_t t2;

	struct arg_set arg1;
	struct arg_set arg2;

	int reports_in = 0;
	int total_words = 0;

	pthread_mutex_lock(&lock); // lock the mail box now

	arg1.file_name = av[1];
	arg1.count = 0;
	pthread_create(&t1, NULL, count_words, (void *)&arg1);

	arg2.file_name = av[2];
	arg2.count = 0;
	pthread_create(&t2, NULL, count_words, (void *)&arg2);

	while (reports_in < 2) {
		printf("MAIN: waiting for flag to go up\n");
		pthread_cond_wait(
			&flag,
			&lock); // suspend the mian thread and wait for the signal
		printf("MAIN: wow! flag was raised, I have the lock\n");
		assert(mail_box != NULL);
		printf("%d: %s\n", mail_box->count, mail_box->file_name);
		total_words += mail_box->count;
		if (mail_box == &arg1) {
			pthread_join(t1, NULL);
		}
		if (mail_box == &arg2) {
			pthread_join(t2, NULL);
		}
		mail_box = NULL;
		pthread_cond_signal(&flag);
		reports_in += 1;
	}

	printf("%d: total words\n", total_words);
	return 0;
}

void *count_words(void *arg)
{
	struct arg_set *a = (struct arg_set *)arg;
	FILE *fp = NULL;
	int c = EOF;
	int prev = '\0';

	if ((fp = fopen(a->file_name, "r")) != NULL) {
		while ((c = getc(fp)) != EOF) {
			if (!isalnum(c) && isalnum(prev)) {
				a->count += 1;
			}
			prev = c;
		}
		fclose(fp);
	} else {
		perror(a->file_name);
	}

	printf("COUNT: waiting to get lock\n");
	pthread_mutex_lock(&lock);
	printf("COUNT: have lock, storing data\n");
	if (mail_box != NULL) {
		pthread_cond_wait(&flag, &lock);
	}
	mail_box = a;

	printf("COUNT: raising flag\n");
	pthread_cond_signal(&flag);
	printf("COUNT: unlocking box\n");
	pthread_mutex_unlock(&lock);
	return NULL;
}
