//
// Created by Chen Chen on 2023/6/20.
//

#ifndef KILO_ROW_H
#define KILO_ROW_H

#include "data.h"
#include <stdlib.h>

int editorRowCxToRx(erow *row, int cx);
void editorAppendRow(char *s, size_t len);

#endif//KILO_ROW_H
