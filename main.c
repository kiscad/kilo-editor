#include "input.h"
#include "output.h"
#include "terminal.h"

int main() {
  enableRawMode();
  initEditor();

  while (1) {
    editorRefreshScreen();
    editorProcessKeypress();
  }
  return 0;
}
