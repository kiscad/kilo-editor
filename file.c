//
// Created by Chen Chen on 2023/6/19.
//

#include "file.h"
#include "data.h"
#include "error.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void editorOpen(char *filename) {
  FILE *fp = fopen(filename, "r");
  if (!fp) die("fopen");

  char *line = NULL;
  size_t linecap = 0;// zero line-capacity cause allocating new memory
  ssize_t linelen;
  linelen = getline(&line, &linecap, fp);
  if (linelen != -1) {
    while (linelen > 0 && (line[linelen - 1] == '\n' || line[linelen - 1] == '\r'))
      linelen--;
    E.row.size = linelen;
    E.row.chars = malloc(linelen + 1);
    memcpy(E.row.chars, line, linelen);
    E.row.chars[linelen] = '\0';
    E.numrows = 1;
  }
  free(line);
  fclose(fp);
}
