#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CArgs {
  char *input;
  char *output;
  double scale;
} CArgs;

struct CArgs parse_args(void);
