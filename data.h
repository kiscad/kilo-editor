//
// Created by Chen Chen on 2023/6/14.
//

#ifndef KILO_DATA_H
#define KILO_DATA_H

#include <termios.h>
#include <time.h>

typedef struct erow {
  int size;
  char *chars;
  int rsize;   // length of render string
  char *render;// the actual chars to draw on screen
} erow;

struct editorConfig {
  int cx, cy;// E.cx is an index into the `chars` field
  int rx;    // E.rx is and index into the `render` field
  int rowoff;// row offset
  int coloff;// column offset
  int screenrows;
  int screencols;
  int numrows;
  erow *row;
  char *filename;
  char statusmsg[80];
  time_t statusmsg_time;
  struct termios orig_termios;
};

struct editorConfig E;

#endif//KILO_DATA_H
