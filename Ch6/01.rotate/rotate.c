/* 
  rotate.c: map a->b, b->c, c->d ... x->y, y->z, z->a
  purpose: useful for showing tty modes
*/
#include <stdio.h>
#include <ctype.h>

int main()
{
	int c = -1;
	while (EOF != (c = getchar())) {
		if (c == 'z') {
			c = 'a';
		} else if (islower(c)) {
			c += 1;
		}
		putchar(c);
	}
	return 0;
}