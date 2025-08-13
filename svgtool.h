#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum COption_u32_Tag {
  Some_u32,
  None_u32,
} COption_u32_Tag;

typedef struct COption_u32 {
  COption_u32_Tag tag;
  union {
    struct {
      uint32_t some;
    };
  };
} COption_u32;

typedef struct CArgs {
  char *input;
  char *output;
  struct COption_u32 width;
  struct COption_u32 height;
  double scale;
  double dpi;
} CArgs;

struct CArgs parse_args(void);
