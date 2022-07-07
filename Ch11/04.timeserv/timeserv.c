/*
 * timeserv.c: a socket-based time of day server
 *
 *
 * Here, I bind the socket to the loopback address
 *
 * $ gcc timeserv.c -o tiemserv && ./timeserv&
 * $ telnet 127.0.0.1 13000
 * Trying 127.0.0.1...
 * Connected to 127.0.0.1.
 * Escape character is '^]'.
 * Wow! got a call
 * The time here is ..Fri May 20 15:40:26 2022
 * Connection closed by foreign host.
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netdb.h>
#include <time.h>
#include <string.h>

#define PORTNUM 13000 // server port number

#define oops(msg)            \
	{                    \
		perror(msg); \
		exit(1);     \
	} // error handling

int main()
{
	// get a socket
	int serv_fd = socket(AF_INET, SOCK_STREAM, 0);
	if (serv_fd == -1) {
		oops("socket");
	}

	// bind an address
	struct sockaddr_in sockaddr;
	memset(&sockaddr, 0, sizeof(sockaddr));
	sockaddr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
	sockaddr.sin_port = htons(PORTNUM);
	sockaddr.sin_family = AF_INET;
	if (bind(serv_fd, (struct sockaddr *)&sockaddr, sizeof(sockaddr)) !=
	    0) {
		oops("bind");
	}

	// activiate server
	if (listen(serv_fd, 1) != 0) {
		oops("listen");
	}

	// main loop: accept request and write
	int client_fd = -1;
	FILE *client_fp = NULL;
	time_t the_time = 0;
	while (1) {
		client_fd = accept(serv_fd, NULL, NULL);
		printf("Wow! got a call\n");

		if (client_fd == -1) {
			oops("accept");
		}

		client_fp = fdopen(client_fd, "w");
		if (client_fp == NULL) {
			oops("fdopen");
		}

		the_time = time(NULL);

		fprintf(client_fp, "The time here is ..");
		fprintf(client_fp, "%s", ctime(&the_time));
		fclose(client_fp);
	}
}
