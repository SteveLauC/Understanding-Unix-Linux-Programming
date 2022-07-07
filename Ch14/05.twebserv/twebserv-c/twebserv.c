/*
 * twebserv.c: a threaded minimal web server(version 2.0)
 * usage: ./twebserv portnumber
 *
 * features: 
 *      supports the GET command only
 *      runs in the current directory
 *      creates a thread to handle each request
 *      supports a special status URL to report internal state
*/

#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <string.h>
#include <stdint.h>

#include <pthread.h>
#include <stdlib.h>
#include <unistd.h>

#include <dirent.h>
#include <time.h>

#include "socklib.h"

void setup(pthread_attr_t *);
void *handle_call(void *);
void process_rq(char *, int);
void sanitize(char *);
int is_built_in(char *);
void built_in(int);
void http_reply(FILE *, int, char *, char *, char *);
void not_implemented(int);
void do_404(int fd);
int isadir(char *);
int not_exist(char *);
void do_ls(char *, int);
char *file_type(char *);
void do_cat(char *, int);

// server facts here
time_t server_started;
int server_requests;

int main(int ac, char *av[])
{
	if (ac != 2) {
		fprintf(stderr, "usage: ./twebserv port_num");
		exit(1);
	}

	int server_fd = make_server_socket((uint16_t)atoi(av[1]), 1);
	if (server_fd == -1) {
		perror("making socket");
		exit(2);
	}

	pthread_t worker;
	pthread_attr_t attribute;
	setup(&attribute);

	int client_fd = 0;
	int *fd_ptr = NULL;
	// main loop: take call, handle call in a new thread
	while (1) {
		client_fd = accept(server_fd, NULL, NULL);
		server_requests += 1;
		fd_ptr = malloc(sizeof(int));
		*fd_ptr = client_fd;
		pthread_create(&worker, NULL, handle_call, (void *)fd_ptr);
	}
}

/*
 * purpose: initialize the status variable
 *      and set the thread attribute to detached
*/
void setup(pthread_attr_t *attrp)
{
	pthread_attr_init(attrp);
	pthread_attr_setdetachstate(attrp, PTHREAD_CREATE_DETACHED);

	time(&server_started);
	server_requests = 0;
}

void *handle_call(void *fd_ptr)
{
	int client_fd = *(int *)fd_ptr;
	free(fd_ptr);
	FILE *client_buf_reader = fdopen(client_fd, "r");
	char buf[BUFSIZ];
	fgets(buf, BUFSIZ, client_buf_reader);

	printf("got a call on %d: request = %s\n", client_fd, buf);

	process_rq(buf, client_fd);
	fclose(client_buf_reader);
	return NULL;
}

void process_rq(char *rq, int client_fd)
{
	char cmd[BUFSIZ];
	char arg[BUFSIZ];
	memset(cmd, 0, BUFSIZ);
	memset(arg, 0, BUFSIZ);

	// request parsing
	if (sscanf(rq, "%s %s", cmd, arg) != 2) {
		return;
	}

	// remove the first `/` in arg
	for (int i = 0; arg[i] != '\0'; i += 1) {
		arg[i] = arg[i + 1];
	}

	if (strcmp(cmd, "GET") != 0) {
		not_implemented(client_fd);
	} else if (is_built_in(arg)) {
		built_in(client_fd);
	} else if (not_exist(arg)) {
		do_404(client_fd);
	} else if (isadir(arg)) {
		do_ls(arg, client_fd);
	} else {
		do_cat(arg, client_fd);
	}
}
int is_built_in(char *arg)
{
	return strncmp(arg, "status", strlen("status")) == 0;
}
void built_in(int fd)
{
	FILE *fp = fdopen(fd, "w");

	http_reply(fp, 200, "OK", "text/palin", NULL);
	fprintf(fp, "Server started: %s\n", ctime(&server_started));
	fprintf(fp, "Total requests: %d\n", server_requests);

	fclose(fp);
}

void http_reply(FILE *fp, int code, char *msg, char *type, char *contents)
{
	if (fp != NULL) {
		fprintf(fp, "HTTP/1.1 %d %s\r\n", code, msg);
		fprintf(fp, "Content-type: %s\r\n\r\n", type);
		if (contents != NULL) {
			fprintf(fp, "%s\r\n", contents);
		}
	}
	fflush(fp);
}

void not_implemented(int fd)
{
	FILE *fp = fdopen(fd, "w");
	http_reply(fp, 501, "Not Implemented", "text/plain",
		   "That command is not implemented yet");
	fclose(fp);
}

void do_404(int fd)
{
	FILE *fp = fdopen(fd, "w");
	http_reply(fp, 404, "Not Found", "text/plain",
		   "That item you seek is not here");
	fclose(fp);
}

int isadir(char *f)
{
	struct stat info;
	return (stat(f, &info) != -1 && S_ISDIR(info.st_mode));
}

int not_exist(char *f)
{
	struct stat info;
	return stat(f, &info) == -1;
}

void do_ls(char *dir, int fd)
{
	FILE *fp = fdopen(fd, "w");
	DIR *dir_ptr = NULL;
	struct dirent *dirent_ptr = NULL;

	http_reply(fp, 200, "OK", "text/plain", NULL);
	fprintf(fp, "Listing of directory %s\n", dir);

	if ((dir_ptr = opendir(dir)) != NULL) {
		while ((dirent_ptr = readdir(dir_ptr)) != NULL) {
			fprintf(fp, "%s\n", dirent_ptr->d_name);
		}

		closedir(dir_ptr);
	}
	fclose(fp);
}

char *file_type(char *f)
{
	char *idx = strchr(f, '.');
	if (idx == NULL) {
		return "";
	} else {
		return idx + 1;
	}
}

void do_cat(char *file, int fd)
{
	char *extension = file_type(file);
	char *content_type = "text/plain";
	FILE *client_writer = fdopen(fd, "w");
	FILE *file_reader = fopen(file, "r");

	if (strcmp(extension, "html") == 0) {
		content_type = "text/html";
	} else if (strcmp(extension, "gif") == 0) {
		content_type = "image/gif";
	} else if (strcmp(extension, "jpg") == 0) {
		content_type = "image/jpg";
	} else if (strcmp(extension, "jpeg") == 0) {
		content_type = "image/jpeg";
	}

	if (client_writer != NULL && file_reader != NULL) {
		http_reply(client_writer, 200, "OK", content_type, NULL);

		int c = EOF;
		while ((c = fgetc(file_reader)) != EOF) {
			putc(c, client_writer);
		}
		fclose(client_writer);
		fclose(file_reader);
	}
}
