/*
 * lserv1.c: license server server program version 1
*/

#include <netinet/in.h>
#include <stdio.h>
#include <sys/socket.h>
#include <sys/errno.h>
#include "lserv_funcs1.h"

#define MSG_LEN 128
int main() {
    struct sockaddr_in client_addr;
    socklen_t addrlen = sizeof(client_addr);
    char buf[MSG_LEN];
    int ret = -1;
    int server_sock = setup();

    while (1) {
        addrlen = sizeof(client_addr);
        ret = recvfrom(server_sock, buf, MSG_LEN, 0, (struct sockaddr *)&client_addr, &addrlen);
        if (ret != -1) {
            buf[ret] = '\0'; // make it nul-terminated
            narrate("GOT: ", buf, &client_addr);
            handle_request(buf, &client_addr, addrlen);
        } else if (errno == EINTR) { // when the receive is inturrupted by a signal
            perror("revcfrom");
        }
    }
}
