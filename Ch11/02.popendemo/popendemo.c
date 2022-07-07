/*
 * popendemo.c: demostrates how to open a program for standard I/O
 *
 *
 * import points:
 *      1. `popen()` returns a `FILE *`, just like `fopen()`
 *      2. the `FILE *` it returns can be read or write with all the standards
 *         functions
 *      3. you need to use `pcolse()` to close the stream when done
*/
#include <stdio.h>
#include <stdlib.h>

int main()
{
	FILE *fp = NULL;
	char buf[100] = { '\0' };
	int i = 0;

	fp = popen("who|sort", "r");

	while (fgets(buf, 100, fp) != NULL) {
		printf("%3d %s", i++, buf);
	}

	pclose(fp);
	return 0;
}
