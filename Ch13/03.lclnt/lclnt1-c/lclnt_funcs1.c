/*
 * lclnt_funcs1.c: functions for the client of licence client
 *
 * acquire a licence:
 *      client: HELO pid
 *      server: TICK ticket -string(on successs) FAIL failure -msg(on failure)
 * release a licence:
 *      client: GBYE ticket -string
 *      server: THNX info -string(on success) FAIL error -string(on failure)
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <netdb.h>
#include "dgram.h"

static int pid = -1;                 // process id
static int sd = -1;                  // socket fd
static struct sockaddr server_addr;  // server address
static socklen_t serv_len;           // server address len
static char ticket_buf[64];         // buffer to hold our ticket
static int have_ticket = 0;          // set when we have a ticket(state indicator)

#define MSGLEN 128                   // size of our datagram
#define SERVER_PORTNUM 2020          // server port number
#define HOSTLEN 64                   // hostname length
#define oops(msg){perror(msg); exit(1);} // error handling function

/*
 * purpose: get pid, socket and address of licence server
 *
 * note: assume license server is on the same host with client
*/
void setup(){
    pid = getpid();

    sd = make_dgram_client_socket();
    if (sd == -1) {
        oops("can not create socket");
    }
    char hostname[HOSTLEN];
    memset(hostname, 0, HOSTLEN);
    gethostname(hostname, HOSTLEN);
    make_internet_address(hostname, SERVER_PORTNUM, (struct sockaddr_in *)&server_addr);
    serv_len = sizeof(server_addr); 
}

/*
 * purpose: shut down the licence client
 *
 * action: close the client socket file descriptor
*/
void shut_down() {
    close(sd);
}

void syserr(char * msg){
    char buf[MSGLEN];
    sprintf(buf, "CLIENT[%d]: %s\n", pid, msg);
    perror(buf);
}

/*
 * purpose: print message to stderr for debugging and demo purposes
 *
 * arguments: 
 *      * `msg1`: message to be printed
 *      * `msg2`: message to be printed
*/
void narrate(char * msg1, char * msg2) {
    fprintf(stderr, "CLIENT [%d]: %s %s\n", pid, msg1, msg2);
}

/*
 * purpose: send a request to the server and get a response back
 *
 * arguments:
 *      * `msg`: message to be sent
 *
 * return: response
*/
char * do_transacion(char * msg) {
    // send message to the server
    int ret = sendto(sd, msg, strlen(msg), 0, (struct sockaddr *)&server_addr, serv_len); 
    if (ret == -1) {
        syserr("sendto");
        return NULL;
    }

    static char buf[MSGLEN];
    // get response
    ret = recvfrom(sd, buf, sizeof(buf), 0, NULL, NULL);
    if (ret == -1) {
        syserr("sendto");
        return NULL;
    }

    return buf;
}

/*
 * purpose: get a ticket from the licence server
 *
 * return: 0 on success, -1 on failure
*/
int get_ticket(){
    // don't be greedy
    if (have_ticket) {
        return 0;
    }
    char buf[MSGLEN];
    sprintf(buf, "HELO %d", pid);
    char * response = do_transacion(buf);
    if (response == NULL) {
        return -1;
    }

    if (strncmp(buf, "TICK", 4) == 0) {
        strcpy(ticket_buf, response+5); // copy "ticket -string" to ticket_buf
        have_ticket = 1;
        narrate("got a ticket", ticket_buf);
        return 0;
    }

    if (strncmp(buf, "FAIL", 4) == 0) {
        narrate("Could not get a ticket", response);
    } else {
        narrate("unknown message", response);
    }

    return -1;
}

/*
 * purpose: give a ticket back to the server
 *
 * return: 0 on success, -1 on failure
*/
int release_ticket() {
    if (!have_ticket) {
        return 0;
    }
    char buf[MSGLEN]; 
    sprintf(buf, "GBYE %s", ticket_buf);
    char * response = do_transacion(buf);
    if (response == NULL) {
        return -1;
    }

    if (strncmp(response, "THNX", 4) == 0) {
        narrate("released ticket ok", "");
        return 0;
    }
    if (strncmp(response, "FAIL", 4) == 0) {
        narrate("release failed", response+5);
    }else {
        narrate("unknown message", response);
    }

    return -1;
}
