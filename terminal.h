//
// Created by Chen Chen on 2023/6/12.
//

#ifndef KILO_TERMINAL_H
#define KILO_TERMINAL_H

#include <termios.h>

struct editorConfig {
  int cx, cy;
  int screenrows;
  int screencols;
  struct termios orig_termios;
};
struct editorConfig E;

enum editorKey {
  ARROW_LEFT = 1000,
  ARROW_RIGHT,
  ARROW_UP,
  ARROW_DOWN,
  PAGE_UP,
  PAGE_DOWN,
  HOME_KEY,
  END_KEY,
  DEL_KEY,
};

void enableRawMode();
void disableRawMode();
int editorReadKey();
void initEditor();

#endif//KILO_TERMINAL_H
