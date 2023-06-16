//
// Created by Chen Chen on 2023/6/14.
//

#ifndef KILO_DATA_H
#define KILO_DATA_H

#include <termios.h>

typedef struct erow {
  int size;
  char *chars;
} erow;

struct editorConfig {
  int cx, cy;
  int screenrows;
  int screencols;
  int numrows;
  erow row;
  struct termios orig_termios;
};

struct editorConfig E;

#endif//KILO_DATA_H
