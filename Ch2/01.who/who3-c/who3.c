/*
 * who3.c - who with buffered reads
 *        - supresses empty records
 *        - formats time nicely
 *        - buffers input (using utmplib.c)
*/

#include <time.h>
#include <string.h>
#include <ctype.h>
#include "utmplib.h"

void show_info(struct utmp *);
void showtime(time_t);

/*
 * trim the string s
*/
char *trim(char *s)
{
	char *ptr;
	if (!s)
		return NULL; // handle NULL string
	if (!*s || strlen(s) == 0)
		return s; // handle empty string
	for (ptr = s + strlen(s) - 1; (ptr >= s) && isspace(*ptr); --ptr)
		;
	ptr[1] = '\0';
	return s;
}

/*
 * display time in a format fit for human consumption
*/
void show_time(int seconds)
{
	time_t sec = (time_t)seconds;
	time_t *p = &sec;
	char *s = trim(ctime(p));
	printf("%s", s + 4);
}

/*
 * display contents of the utmp struct in human readable format
 * display nothing if the record is empty
*/
void show_info(struct utmp *ut_buf_p)
{
	if (ut_buf_p->ut_type != USER_PROCESS) {
		return;
	}

	printf("%-8.8s", ut_buf_p->ut_user);
	printf(" ");
	printf("%-8.8s", ut_buf_p->ut_line);
	printf(" ");
	show_time(ut_buf_p->ut_tv.tv_sec);
	if (ut_buf_p->ut_host[0] != '\0') {
		printf("(%s)", ut_buf_p->ut_host);
	}
	printf("\n");
}

int main()
{
	struct utmp *utbufp = NULLUT;
	if (utmp_open("/var/run/utmp") == -1) {
		perror("/var/run/utmp");
		exit(-1);
	}

	while ((utbufp = utmp_next()) != NULL) {
		show_info(utbufp);
	}

	utmp_close();
	return 0;
}