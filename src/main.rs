use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use im::vector;

pub mod editor;
pub mod rope;

use editor::*;
use rope::{Rope, RopeMarkers};

const PADDING : f64 = 5.0;

fn main() -> Result<(), PlatformError> {
  // TODO - Find a more idiomatic way to calculate the size
  let char_width = 9.0;
  let width = 80.0 * char_width + 1.0 + 2.0 * PADDING;
  let height = 800.0;

  let main_window = WindowDesc::new(ui_builder)
    .window_size((width, height));

  let default_str = Rope { text: "0123456789\n".repeat(8) };

  AppLauncher::with_window(main_window)
    .use_simple_logger()
    .launch(EditorState { text: default_str, cursors: RopeMarkers { markers: vector![0] } })
}

fn ui_builder() -> impl Widget<EditorState> {
  EditorWidget::new()
    .padding(PADDING)
}
