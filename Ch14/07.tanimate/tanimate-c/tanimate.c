/*
 * tanimate.c: animate several strings using threads, curses
*/

#include <stdio.h>
#include <curses.h>
#include <pthread.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

#define MAXMSG 10   // limit to the number strings
#define TUNIT 20000 // timeunits in microseconds

struct prop_set {
    char * str;
    int row;
    int delay;
    int dir;
};

int setup(int, char **, struct prop_set *);
void * animate(void *);


pthread_mutex_t mx = PTHREAD_MUTEX_INITIALIZER;

int main(int ac, char * av[]) {
    int c;                          // user input
    pthread_t threads[MAXMSG];      // the threads
    struct prop_set props[MAXMSG];  // properities of string
    int num_msg;                    // number of strings
    int i;

    if (ac == 1) {
        printf("usage: tanimate string..\n");
        exit(1);
    }

    num_msg = setup(ac-1, av+1, props);

    for(i = 0; i < num_msg; i++) {
        if(pthread_create(&threads[i], NULL, animate, &props[i])) {
            fprintf(stderr, "error creating thread");
            endwin();
            exit(0);
        }
    }

    while (1) {
        c = getch();
        if (c == 'Q') break;
        if (c == ' ') {
            for(int j = 0; j < num_msg; j++) {
                props[j].dir *= -1;
            }
        }
        if (c >= '0' && c <= '9') {
            int j = c - '0';
            if (j < num_msg ) {
                props[j].dir *= -1;
            }
        }
    }

    pthread_mutex_lock(&mx);
    for(int i = 0; i < num_msg; i++) {
        pthread_cancel(threads[i]);
    }
    endwin();
    return 0;
}

int setup(int n_strings, char * strings[], struct prop_set prop_s[]) {
    int num_msg = n_strings > MAXMSG ? MAXMSG : n_strings;
    int i;
    
    srand((unsigned int)getpid());

    // assign rows and velocities to each string
    for(i = 0; i < num_msg; i++) {
        prop_s[i].str = strings[i];
        prop_s[i].row = i;
        prop_s[i].delay = 1 + (rand()%15);
        prop_s[i].dir = (rand()%2? 1 : -1);
    }

    // set up curses
    initscr();
    crmode();
    noecho();
    clear();
    mvprintw(LINES-1, 0, "'Q' to quit, '0'..'%d' to bounce", num_msg-1);

    return num_msg;
}

// the code that runs in each thread
void * animate(void *arg) {
    struct prop_set * info = (struct prop_set*)arg; 

    int len = strlen(info->str) + 2;
    int col = rand() % (COLS - len -3);

    while (1) {
        usleep(info->delay * TUNIT);

        // critical section
        pthread_mutex_lock(&mx);
            move(info->row, col);
            addch(' ');
            addstr(info->str);
            addch(' ');
            move(LINES-1, COLS-1);
            refresh();
        pthread_mutex_unlock(&mx);

        col += info -> dir;

        if (col <= 0 && info -> dir == -1) {
            info->dir = 1;
        }

        if (col + len >= COLS && info->dir == 1) {
            info->dir = -1;
        }
    }
}
