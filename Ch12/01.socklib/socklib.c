/*
 * socklib.c: This file contains functions used lots when writing internet 
 *            client/server programs. 
 *
 * The two main function here are:
 *     1. int make_server_socket(int port_num, int backlog);
 *     return a server socket or -1 if on error
 *     2. int connect_to_server(char * hostname, int port_num); 
 *     return a connected socket or -1 if on error
*/

#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netdb.h>
#include <string.h>
#include <stdint.h>

#define HOSTNAME_LEN 64
#define BACKLOG 1

int make_server_socket(uint16_t port_num, int backlog);
int connect_to_server(char *hostname, uint16_t port_num);

int make_server_socket(uint16_t port_num, int backlog)
{
	// get a socket
	int server_fd = socket(AF_INET, SOCK_STREAM, 0);
	if (-1 == server_fd) {
		return -1;
	}

	// bind a address
	struct sockaddr_in server_address;
	memset(&server_address, 0, sizeof(server_address));
	server_address.sin_port = htons(port_num);
	server_address.sin_family = AF_INET;

	char hostname[HOSTNAME_LEN];
	memset(hostname, 0, HOSTNAME_LEN);
	gethostname(hostname, HOSTNAME_LEN);
	struct hostent *hp = gethostbyname(hostname);
	if (NULL == hp || AF_INET != hp->h_addrtype) {
		return -1;
	}
	memcpy(&server_address.sin_addr, hp->h_addr_list[0], hp->h_length);
	if (bind(server_fd, (struct sockaddr *)&server_address,
		 sizeof(server_address)) != 0) {
		return -1;
	}

	// activate the server
	if (listen(server_fd, backlog) != 0) {
		return -1;
	}

	return server_fd;
}

int connect_to_server(char *hostname, uint16_t port_num)
{
	// get a socket
	int client_fd = socket(AF_INET, SOCK_STREAM, 0);
	if (client_fd == -1) {
		return -1;
	}

	// connect to the server
	struct hostent *hp = gethostbyname(hostname);
	if (NULL == hp || AF_INET != hp->h_addrtype) {
		return -1;
	}
	struct sockaddr_in server_address;
	memset(&server_address, 0, sizeof(server_address));
	memcpy(&server_address.sin_addr, hp->h_addr_list[0], hp->h_length);
	server_address.sin_family = AF_INET;
	server_address.sin_port = htons(port_num);
	if (connect(client_fd, (struct sockaddr *)&server_address,
		    sizeof(server_address)) != 0) {
		return -1;
	}

	return client_fd;
}
