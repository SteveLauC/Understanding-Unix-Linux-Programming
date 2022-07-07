#include <stdio.h>

int main(int ac, char **av)
{
	for (int i = 0; i < ac; i += 1) {
		printf("args[%d] %s\n", i, av[i]);
	}

	fprintf(stderr, "This message is sent to stderr\n");

	return 0;
}
