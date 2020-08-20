use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use im::vector;

pub mod editor;
pub mod rope;
pub mod cursors;

use editor::*;
use rope::{Rope, RopeMarkers};

const PADDING : f64 = 5.0;

fn main() -> Result<(), PlatformError> {
  // TODO - Find a more idiomatic way to calculate the size
  let char_width = 9.0;
  let width = 80.0 * char_width + 1.0 + 2.0 * PADDING;
  let height = 800.0;

  let editor_state = EditorState {
    text: Rope { text: "12345\n123\n".repeat(5) },
    cursors: RopeMarkers { markers: vector![0, 1] },
  };
  let editor_state2 = editor_state.clone();

  let ui_builder = move || EditorWidget::new(&editor_state)
    .padding(PADDING)
  ;

  let main_window = WindowDesc::new(ui_builder)
    .window_size((width, height));

  AppLauncher::with_window(main_window)
    .use_simple_logger()
    .launch(editor_state2)
}
