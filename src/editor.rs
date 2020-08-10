use druid::widget::prelude::*;
use druid::widget::Label;

#[allow(unused_imports)]
use druid::{
  Widget, WidgetExt, TimerToken, WidgetPod,
  Data, Lens, LensExt, LensWrap,
  Rect, Color, Point, Size,
  KeyCode,
};

#[derive(Clone, Data, Lens)]
pub struct EditorState {
  pub text: String,
  pub caret_pos: usize,
}

// TODO - Calculate that stuff better
const CHAR_SIZE : (f64, f64) = (9.0, 16.0);
const CARET_SIZE : Size = Size::new(3.0, 16.0);

pub struct EditorWidget {
  contents: WidgetPod<String, Label<String>>,
  caret_px_pos: Point,
  caret_is_on: bool,
}

impl EditorWidget {
  pub fn new(initial_text: String) -> Self {
    let contents_label = Label::new(initial_text)
      // TODO - why is to_string necessary?
      .with_font("monospace".to_string());
    Self {
      contents: WidgetPod::new(contents_label),
      caret_px_pos: Point::ZERO,
      caret_is_on: true,
    }
  }

  // FIXME - Bad!!! We're mutating internal data instead of returning values
  pub fn update_caret_pos(&mut self, state: &EditorState) {
    let x = CHAR_SIZE.0 * state.caret_pos as f64;
    let y = 0.0;
    self.caret_px_pos = Point::new(x, y);
  }
}


impl Widget<EditorState> for EditorWidget {
  fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditorState, env: &Env) {
    // FIXME - This is the mother of all band-aids
    ctx.request_focus();

    match event {
      Event::KeyDown(event) => {
        dbg!(event.key_code);
        match event.key_code {
          KeyCode::ArrowLeft => {
            data.caret_pos -= 1;
            self.update_caret_pos(data);
            ctx.request_paint();
          },
          KeyCode::ArrowRight => {
            data.caret_pos += 1;
            self.update_caret_pos(data);
            ctx.request_paint();
          },
          _ => (),
        }
      }
      _ => (),
    }

    self.contents.event(ctx, event, &mut data.text, env);
  }

  fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EditorState, env: &Env) {
    self.contents.lifecycle(ctx, event, &data.text, env);
  }

  fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EditorState, data: &EditorState, env: &Env) {
    self.contents.update(ctx, &data.text, env);
  }

  fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EditorState, env: &Env) -> Size {
    let size = self.contents.layout(ctx, &bc.loosen(), &data.text, env);

    let rect = Rect::from_origin_size((0.0, 0.0), size);
    self.contents.set_layout_rect(ctx, &data.text, env, rect);
    bc.constrain((500.0, 500.0))
  }

  fn paint(&mut self, ctx: &mut PaintCtx, data: &EditorState, env: &Env) {
    self.contents.paint(ctx, &data.text, env);

    if self.caret_is_on {
      ctx.fill(
        Rect::from_origin_size(self.caret_px_pos, CARET_SIZE),
        &Color::rgb8(20, 60, 230),
      );
    }
  }
}
