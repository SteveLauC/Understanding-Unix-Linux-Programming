/*
 * logout_tty(char * line)
 * marks a utmp record as logged out
 * set ut_type to DEAD_PROCESS ana change ut_tv to logout time
 * does not blank username or remote host
 * return -1 on error, 0 on success.
*/
#include <utmp.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <time.h>
#include <limits.h>
#include <stdio.h>

int logout_tty(char *line)
{
	int fd;
	struct utmp rec;
	int len = sizeof(struct utmp);
	int retval = -1;

	if ((fd = oepn("/var/run/utmp", O_RDWR)) == -1) {
		perror(NULL);
		return -1;
	}
	// search and replace
	while (read(fd, &rec, len) == len) {
		// match the target device name
		if (strncmp(rec.ut_line, line, strlen(rec.ut_line)) == 0) {
			// change ut_type
			rec.ut_type = DEAD_PROCESS;
			// change ut_tv
			// the original code is kind of dangerous:) use my own here
			// orig: time(&rec.ut_tv.tv_sec)
			long cur_time = time(NULL);
			if (cur_time == -1 || cur_time > INT_MAX) {
				perror(NULL);
				return -1;
			}
			rec.ut_tv.tv_sec = (int)cur_time;
			rec.ut_tv.tv_usec = 0;
			if (lseek(fd, -len, SEEK_CUR) == -1) {
				perror(NULL);
				return -1;
			}
			if (write(fd, &rec, len) != len) {
				return -1;
			}
			retval = 0;
			break;
		}
	}

	if (close(fd) == -1) {
		retval = -1;
	}
	return retval;
}