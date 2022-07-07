/*
 * rlsd.c: a remote ls server without paranoia:)
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <netdb.h>
#include <string.h>
#include <assert.h>

#define PORT_NUM 15000
#define HOST_NAME_LEN 64

#define oops(msg)            \
	{                    \
		perror(msg); \
		exit(1);     \
	}

int main()
{
	// get a socket
	int server_fd = socket(AF_INET, SOCK_STREAM, 0);
	if (server_fd == -1) {
		oops("socket");
	}

	// bind an address to the socket
	char host_name[HOST_NAME_LEN];
	memset(host_name, 0, HOST_NAME_LEN);
	gethostname(host_name, HOST_NAME_LEN);

	struct hostent *hp = gethostbyname(host_name);
	if (hp == NULL) {
		oops("gethostbyname");
	}
	assert(hp->h_addrtype == AF_INET); // assert that we get a v4 address
	struct sockaddr_in server_address;
	memset(&server_address, 0, sizeof(server_address));
	memcpy(&server_address.sin_addr, hp->h_addr_list[0], hp->h_length);
	server_address.sin_port = htons(PORT_NUM);
	server_address.sin_family = AF_INET;

	if (bind(server_fd, (struct sockaddr *)&server_address,
		 sizeof(server_address)) != 0) {
		oops("bind");
	}

	// activate the server
	if (listen(server_fd, 1) != 0) {
		oops("listen");
	}

	while (1) {
		int client_fd = accept(server_fd, NULL, NULL);
		if (client_fd == -1) {
			oops("accept");
		}

		// read dir name
		char dir_name[100];
		memset(dir_name, 0, 100);
		FILE *client_reader = fdopen(client_fd, "r");
		if (client_reader == NULL) {
			oops("fdopen");
		}
		fgets(dir_name, 100, client_reader);

		// call popen and execute `ls dir_name`
		char command[150];
		memset(command, 0, 150);
		sprintf(command, "ls %s", dir_name);
		FILE *child_process_reader = popen(command, "r");
		if (child_process_reader == NULL) {
			oops("popen");
		}

		// send result back
		FILE *client_writer = fdopen(client_fd, "w");
		if (client_writer == NULL) {
			oops("fdopen");
		}
		int c = EOF;
		char result[300];
		memset(result, 0, 300);
		while ((c = getc(child_process_reader)) != EOF) {
			putc(c, client_writer);
		}

		pclose(client_writer);
		pclose(client_reader);
		close(client_fd);
	}
}
