/* 
    bounce.h
    some settings for the game
*/

#define BLANK ' '
#define DFL_SYMBOL 'O'

// border
#define TOP_ROW 5
#define BOT_ROW 50
#define LEFT_EDGE 10
#define RIGHT_EDGE 70

// initial position
#define X_INIT 10
#define Y_INIT 10

// framerate
#define TICKS_PER_SEC 50

#define X_TTM 5
#define Y_TTM 8

struct ppball {
	// postion
	int x_pos;
	int y_pos;

	int x_ttm;
	int y_ttm;

	int x_ttg;
	int y_ttg;

	// direction
	int y_dir;
	int x_dir;

	// symbol to represent the ball
	char symbol;
};