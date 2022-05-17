/*
 * tinybc.c: a tiny calculator that uses dc to do its work
 *
 * demostrates biodierctional pipes
 *
 * what tinybc does here: reads user input like `number op number` and converts
 * it to the form `number number op`, passes it to dc and get the calculating
 * result back, then print it to stdout
 *
 *
 * program outline:
 * 	a. parent-process: get two pipes
 * 	b. parent-process: fork(get another process `child-process`)
 * 	c. chld-process: redirect stdin to `pipetodc`, and stdout to
 * `pipefromdc` d. child-process: execute `dc` e. parent-process: get user
 * input, convert the input format to `reverse polish notation` pass the input
 * to the child-process f. child-process: calculation g. parent-process: get the
 * final result and print it to stdout
 */

#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>

#define oops(m, x)                                                             \
  {                                                                            \
    perror(m);                                                                 \
    exit(x);                                                                   \
  }

void fatal(char *);
void be_dc(int *, int *);
void be_bc(int *, int *);

int main() {
  int pid = -1;
  int to_dc[2] = {-1, -1};
  int from_dc[2] = {-1, -1};

  // make two pipes
  if (pipe(to_dc) == -1 || pipe(from_dc) == -1) {
    oops("pipe failed", 1)
  }

  // get a process for user interface
  pid = fork();
  if (pid == -1) {
    oops("can not fork", 2);
  } else if (pid == 0) {
    be_dc(to_dc, from_dc);
  } else {
    be_bc(to_dc, from_dc);
    wait(NULL);
  }
  return 1;
}

/*
 * purpose: set up `dc`
 *
 * action: redirect stdin and stdout, then execl dc
 *
 * argument:
 * 	* `to_dc`: pipe from `tinybc` to `dc`
 * 	* `from_dc: pipe from `dc` to `tinybc`
 */
void be_dc(int to_dc[2], int from_dc[2]) {
  // redirect stdin to to_dc[0]
  if (dup2(to_dc[0], 0) == -1) {
    oops("dc: cannot redirect stdin", 3);
  }
  close(to_dc[0]);
  close(to_dc[1]);

  // redirect stdout to from_dc[1]
  if (dup2(from_dc[1], 1) == -1) {
    oops("dc: cannot redirect stdout", 4);
  }
  close(from_dc[0]);
  close(from_dc[1]);

  // now execl `dc` with `-` option (to make it read from stdin)
  execlp("dc", "dc", "-", NULL);
  oops("cannot run dc", 5);
}

/*
 * purpose: set up `tinybc`
 *
 * action: read from stdin and convert into reverse polish notaion, send down
 * pipe then read from other pipe and print to user
 *
 * argument:
 * 	* `to_dc`: pipe from `tinybc` to `dc`
 * 	* `from_dc: pipe from `dc` to `tinybc`
 *
 * 	note: use fdopen to convert a file descriptor to a stream
 */
void be_bc(int to_dc[2], int from_dc[2]) {
  int num1 = 0;
  int num2 = 0;
  char operation[BUFSIZ] = {'\0'};
  // char operation = 0;
  char message[BUFSIZ] = {'\0'};
  FILE *fpout = NULL;
  FILE *fpin = NULL;

  close(to_dc[0]);   // won't read from pipe to dc
  close(from_dc[1]); // won't wirte to pipe form dc

  fpout = fdopen(to_dc[1], "w");
  fpin = fdopen(from_dc[0], "r");
  if (fpout == NULL || fpin == NULL) {
    fatal("Error converting pipes to streams");
  }

  // main loop
  printf("tinybc: ");
  while (fgets(message, BUFSIZ, stdin) != NULL) {
    if (sscanf(message, "%d %s %d", &num1, operation, &num2) != 3) {
      printf("syntax error\n");
      continue;
    }

    if (fprintf(fpout, "%d\n%d\n%c\np\n", num1, num2, *operation) == EOF) {
      fatal("Error writing");
    }
    fflush(fpout);

    if (fgets(message, BUFSIZ, fpin) == NULL) {
      break;
    }
    printf("%d %c %d = %s", num1, *operation, num2, message);

    printf("tinybc: ");
  }

  fclose(fpout);
  fclose(fpin);
}

void fatal(char *msg) {
  fprintf(stderr, "Error: %s\n", msg);
  exit(1);
}
