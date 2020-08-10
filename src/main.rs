use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};

pub mod editor;

use editor::*;

const PADDING : f64 = 5.0;

fn main() -> Result<(), PlatformError> {
  // TODO - Find a more idiomatic way to calculate the size
  let char_width = 9.0;
  let width = 80.0 * char_width + 1.0 + 2.0 * PADDING;
  let height = 800.0;

  let main_window = WindowDesc::new(ui_builder)
    .window_size((width, height));

  let default_str = "0123456789".repeat(8);

  AppLauncher::with_window(main_window)
    .use_simple_logger()
    .launch(EditorState {text: default_str, caret_pos: 0})
}

fn ui_builder() -> impl Widget<EditorState> {
  let default_str = "0123456789".repeat(8);

  EditorWidget::new(default_str)
    .padding(PADDING)
}
