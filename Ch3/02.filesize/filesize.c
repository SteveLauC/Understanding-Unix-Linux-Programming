// filesize.c - prints size of passwd file

#include <stdio.h>
#include <sys/stat.h>

int main()
{
	struct stat st_buf;
	if (stat("/etc/passwd", &st_buf) == -1) {
		perror("/etc/passwd");
	} else {
		printf("The size of /etc/passwd is %ld\n", st_buf.st_size);
	}
}