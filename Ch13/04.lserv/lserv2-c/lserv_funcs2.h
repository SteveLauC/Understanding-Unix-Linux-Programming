#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>

int setup();
void free_all_tickets();
void shut_down();
void handle_request(char *, struct sockaddr_in *, socklen_t);
char *do_hello(char *);
char *do_goodbye(char *);
void narrate(char *, char *, struct sockaddr_in *);
int is_pid_alive(pid_t);
void ticket_reclaim();
