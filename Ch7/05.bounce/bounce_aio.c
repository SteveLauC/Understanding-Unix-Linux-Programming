/*
    bounce_aio.c
    purpose: animation with user control, using aio_read() etc.
    note: set_ticker() sends SIGALRM, handler does animation
          keyboard sends SIGIO, main only calls pause()
    compile: gcc bounce_aio.c set_ticker.c -l curses -o bounce_aio
*/

#include <stdio.h>
#include <aio.h>
#include <curses.h>
#include <signal.h>
#include <unistd.h>
#include <string.h>

#define MSG "hello"
#define BLK "     "

int row = 10;
int col = 0;
int dir = 1;
int delay = 200;
int done = 0;

struct aiocb kbcbuf;

void on_alarm(int signum);
void on_input(int signum);
int set_ticker(int n_msecs);
void setup_aio_buffer();


int main() {
    initscr();
    crmode();
    noecho();
    clear();  

    signal(SIGIO, on_input);

    // set up buffer and place a request
    setup_aio_buffer();
    aio_read(&kbcbuf);
    signal(SIGALRM, on_alarm);

    mvaddstr(row, col, MSG);
    refresh();
    set_ticker(delay);

    while(!done) {
        pause();
    }
    endwin();
    return 0;
}

void on_alarm(int signum) {
    // printf("debug: call on_alarm()\n");
    signal(SIGALRM, on_alarm);
    mvaddstr(row, col, BLK);
    col+=dir;
    mvaddstr(row, col, MSG);
    refresh();

    // handle borders
    if (col <= 0 && dir==-1) {
        dir *= -1;
    }

    if (col+strlen(MSG)>=COLS && dir == 1) {
        dir *= -1;
    }
}

void on_input(int signum) {
    // fprintf(stderr, "debug: call on_input()\n");
    int c = -1;
    char *cp = (char *) kbcbuf.aio_buf;
    // check for error
    if (aio_error(&kbcbuf) != 0) {
        perror("reading failed");
    } else {
        // get number of chars read 
        if (aio_return(&kbcbuf) == 1) {
            // printf("debug: get a char\n");
            c = *cp;

            if (c == 'Q' || c == EOF) {
                done = 1;
            } else if (c == ' ') {
                dir *= -1;
            }
        }
    }

    // place a new request
    aio_read(&kbcbuf);
}

/*
    set memners of struct 
    First specify ars like those for read(fd, buf, num) and offset;
    Then specify what to do (send signal) and what signal(SIGIO)
*/
void setup_aio_buffer() {
    char buf[1] = {0};
    kbcbuf.aio_fildes = 0;
    kbcbuf.aio_buf = buf;
    kbcbuf.aio_nbytes = 1;
    kbcbuf.aio_offset = 0;

    kbcbuf.aio_sigevent.sigev_notify = SIGEV_SIGNAL;
    kbcbuf.aio_sigevent.sigev_signo = SIGIO;
}
