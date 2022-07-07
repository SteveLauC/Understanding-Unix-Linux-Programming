/* 
 * fileinfo.c 
 * use stat() to obtain and print file properties
 * some members are just numbers
*/

#include <stdio.h>
#include <sys/stat.h>

// displays some info from stat in a `name: value` format
void show_stat_info(char *fname, struct stat *buf)
{
	printf("mode: %o\n", buf->st_mode);
	printf("links: %ld\n", buf->st_nlink);
	printf("user: %d\n", buf->st_uid);
	printf("group: %d\n", buf->st_gid);
	printf("size: %ld\n", buf->st_size);
	printf("mtime: %ld\n", buf->st_mtim.tv_sec);
	printf("name: %s\n", fname);
}

int main(int ac, char *av[])
{
	if (ac == 2) {
		struct stat buf;

		if (stat(av[1], &buf) != -1) {
			show_stat_info(av[1], &buf);
		}
	} else {
		return -1;
	}
	return 0;
}