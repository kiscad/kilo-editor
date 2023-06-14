//
// Created by Chen Chen on 2023/6/13.
//

#include "append_buffer.h"
#include "terminal.h"
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define KILO_VER "0.0.1"

void editorDrawRows(struct abuf *ab) {
  int y;
  for (y = 0; y < E.screenrows; y++) {
    if (y == E.screenrows / 3) {
      char welcome[80];
      int welcomelen = snprintf(welcome, sizeof(welcome),
                                "Kilo editor -- version %s", KILO_VER);
      if (welcomelen > E.screencols) welcomelen = E.screencols;

      int padding = (E.screencols - welcomelen) / 2;
      if (padding) {
        abAppend(ab, "~", 1);
        padding--;
      }
      while (padding--) abAppend(ab, " ", 1);

      abAppend(ab, welcome, welcomelen);
    } else {
      abAppend(ab, "~", 1);
    }

    // clear the part of line to the right of the cursor
    abAppend(ab, "\x1b[K", 3);
    if (y < E.screenrows - 1) {
      abAppend(ab, "\r\n", 2);
    }
  }
}

void editorRefreshScreen() {
  struct abuf ab = ABUF_INIT;

  // hide cursor
  abAppend(&ab, "\x1b[?25l", 6);
  // move cursor top-left corner
  abAppend(&ab, "\x1b[H", 3);

  editorDrawRows(&ab);

  // move cursor
  char buf[32];
  snprintf(buf, sizeof(buf), "\x1b[%d;%dH", E.cy + 1, E.cx + 1);
  abAppend(&ab, buf, strlen(buf));

  // show cursor again
  abAppend(&ab, "\x1b[?25h", 6);

  write(STDOUT_FILENO, ab.b, ab.len);
  abFree(&ab);
}
