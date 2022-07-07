/*
 * showenv.c: shows how to read and print environment
*/

#include <stdio.h>

extern char **environ; // points to the array of strings

int main()
{
	for (int i = 0; environ[i]; i += 1) {
		printf("%s\n", environ[i]);
	}
	return 0;
}