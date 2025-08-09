#include "svg_rastor.h"
#include <cairo.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  CArgs args = parse_args();
  printf("owned input: %s\n", args.input);

  cairo_surface_t *surface =
      cairo_image_surface_create(CAIRO_FORMAT_ARGB32, 2048, 2048);
  cairo_t *cr = cairo_create(surface);

  cairo_select_font_face(cr, "serif", CAIRO_FONT_SLANT_NORMAL,
                         CAIRO_FONT_WEIGHT_NORMAL);
  cairo_set_font_size(cr, 256.0);
  cairo_set_source_rgb(cr, 0.0, 0.0, 1.0);

  cairo_move_to(cr, 200.0, 1000.0);
  cairo_show_text(cr, "hello, world");

  cairo_destroy(cr);

  cairo_surface_write_to_png(surface, "hello.png");
  cairo_surface_destroy(surface);
  return 0;
}
