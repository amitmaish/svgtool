#include "gio/gio.h"
#include "glib.h"
#include "glibconfig.h"
#include "svgtool.h"
#include <_stdio.h>
#include <cairo.h>
#include <librsvg/rsvg.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  CArgs args = parse_args();
  printf("%s\n", args.input);

  GError *error = NULL;
  GFile *file = g_file_new_for_path(args.input);
  RsvgHandle *handle = rsvg_handle_new_from_gfile_sync(
      file, RSVG_HANDLE_FLAGS_NONE, NULL, &error);

  rsvg_handle_set_dpi(handle, args.dpi);

  gdouble svg_width, svg_height;
  rsvg_handle_get_intrinsic_size_in_pixels(handle, &svg_width, &svg_height);

  if (svg_width == 0.0) {
    g_printerr("input svg has no width\n");
    exit(1);
  }

  gdouble aspect_ratio = svg_height / svg_width;
  printf("aspect ratio: %f\n", aspect_ratio);

  switch (args.width.tag) {
  case Some_u32:
    switch (args.height.tag) {
    case Some_u32:
      svg_width = args.width.some;
      svg_height = args.height.some;
      break;
    case None_u32:
      svg_width = args.width.some;
      svg_height = args.width.some * aspect_ratio;
      break;
    }
    break;
  case None_u32:
    switch (args.height.tag) {
    case Some_u32:
      svg_width = args.height.some / aspect_ratio;
      svg_height = args.height.some;
      break;
    case None_u32:
      svg_width = svg_width;
      svg_height = svg_width;
      break;
    }
    break;
  }
  svg_width *= args.scale;
  svg_height *= args.scale;

  printf("dimensions: (%f, %f)\n", svg_width, svg_height);

  if (!handle) {
    g_printerr("could not load: %s\n", error->message);
    exit(1);
  }

  cairo_surface_t *surface = cairo_image_surface_create(
      CAIRO_FORMAT_ARGB32, (int)svg_width, (int)svg_height);
  cairo_t *cr = cairo_create(surface);

  RsvgRectangle viewport = {
      .x = 0.0,
      .y = 0.0,
      .width = svg_width,
      .height = svg_height,
  };

  if (!rsvg_handle_render_document(handle, cr, &viewport, &error)) {
    g_printerr("could not render: %s", error->message);
    exit(1);
  }

  if (cairo_surface_write_to_png(surface, "hello.png") !=
      CAIRO_STATUS_SUCCESS) {
    g_printerr("could not write output file");
    exit(1);
  }

  cairo_destroy(cr);
  cairo_surface_destroy(surface);
  g_object_unref(handle);
  g_object_unref(file);

  return 0;
}
