//
// Created by Chen Chen on 2023/6/14.
//

#ifndef KILO_APPEND_BUFFER_H
#define KILO_APPEND_BUFFER_H

struct abuf {
  char *b;
  int len;
};

#define ABUF_INIT \
  { NULL, 0 }

void abAppend(struct abuf *ab, const char *s, int len);
void abFree(struct abuf *ab);

#endif//KILO_APPEND_BUFFER_H
