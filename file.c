//
// Created by Chen Chen on 2023/6/19.
//

#include "file.h"
#include "error.h"
#include "row.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void editorOpen(char *filename) {
  FILE *fp = fopen(filename, "r");
  if (!fp) die("fopen");

  char *line = NULL;
  size_t linecap = 0;// zero line-capacity cause allocating new memory
  ssize_t linelen;
  while ((linelen = getline(&line, &linecap, fp)) != -1) {
    while (linelen > 0 && (line[linelen - 1] == '\n' || line[linelen - 1] == '\r'))
      linelen--;
    editorAppendRow(line, linelen);
  }
  free(line);
  fclose(fp);
}
