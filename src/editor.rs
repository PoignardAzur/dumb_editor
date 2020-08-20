use std::iter::Iterator;

use druid::widget::prelude::*;
use druid::widget::Label;
use im::{Vector, vector};

#[allow(unused_imports)]
use druid::{
  Widget, WidgetExt, TimerToken, WidgetPod,
  Data, Lens, LensExt, LensWrap,
  Rect, Color, Point, Size,
  KeyCode,
};

use crate::rope::{Rope, RopeMarkers, RopeSpans, RopeData, RopeOps};
use crate::rope::match_spans;
use crate::cursors::{update_cursors, CursorOp};

#[derive(Clone, Data, Lens)]
pub struct EditorState {
  pub text: Rope,
  pub cursors: RopeMarkers,
}

// TODO - Calculate that stuff better
const CHAR_SIZE : (f64, f64) = (9.0, 16.0);
const CURSOR_SIZE : Size = Size::new(3.0, 16.0);

pub struct EditorWidget {
  rows: RopeData<WidgetPod<(), Label<()>>>,
  line_spans: RopeSpans,
  cursors_px_pos: RopeData<Point>,
  cursors_line_index: RopeData<usize>,
  cursors_line_offset: RopeData<usize>,
  cursor_is_on: bool,
}

impl EditorWidget {
  pub fn new(state: &EditorState) -> Self {
    let mut widget = Self {
      rows: RopeData { data: Vec::new() },
      line_spans: RopeSpans { spans: Vector::new() },
      cursors_px_pos: RopeData { data: Vec::new() },
      cursors_line_index: RopeData { data: Vec::new() },
      cursors_line_offset: RopeData { data: Vec::new() },
      cursor_is_on: true,
    };

    widget.update_contents(state);
    widget.update_line_spans(state);
    widget.update_cursor_pos(state);

    widget
  }

  // FIXME - Bad!!! We're mutating internal data instead of returning values
  fn update_cursor_pos(&mut self, state: &EditorState) {
    let cursors_spans = match_spans(&state.cursors, &self.line_spans);
    let cursors_pos = cursors_spans.data.iter()
      .map(|dumb_span_match| {
        let cursor_y = dumb_span_match.index;
        let cursor_x = dumb_span_match.offset;

        Point::new(cursor_x as f64 * CHAR_SIZE.0, cursor_y as f64 * CHAR_SIZE.1)
      })
      .collect();
    let (cursors_index, cursors_offset) = cursors_spans.data.iter()
      .map(|span| (span.index, span.offset))
      .unzip();

    self.cursors_px_pos = RopeData { data: cursors_pos };
    self.cursors_line_index = RopeData { data: cursors_index };
    self.cursors_line_offset = RopeData { data: cursors_offset };
  }

  fn update_line_spans(&mut self, state: &EditorState) {
    let line_spans = state.text.text
      .split("\n")
      .map(|line| line.len() + 1);

    self.line_spans = RopeSpans { spans: line_spans.collect() };
  }

  fn update_contents(&mut self, state: &EditorState) {
    let rows = state.text.text
      .split("\n")
      // TODO - why is to_string necessary?
      .map(|line_str| Label::new(line_str).with_font("monospace".to_string()))
      .map(|line_widget| WidgetPod::new(line_widget));

    self.rows = RopeData { data: rows.collect() };
  }
}


impl Widget<EditorState> for EditorWidget {
  fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditorState, env: &Env) {
    // FIXME - This is the mother of all band-aids
    ctx.request_focus();

    match event {
      Event::KeyDown(event) => {
        match event.key_code {
          KeyCode::ArrowLeft => {
            update_cursors(
              &mut data.cursors,
              &self.cursors_line_index,
              &self.cursors_line_offset,
              &self.line_spans,
              data.text.text.len(),
              CursorOp::MoveLeft,
            );
            ctx.request_paint();
          },
          KeyCode::ArrowRight => {
            update_cursors(
              &mut data.cursors,
              &self.cursors_line_index,
              &self.cursors_line_offset,
              &self.line_spans,
              data.text.text.len(),
              CursorOp::MoveRight,
            );
            ctx.request_paint();
          },
          KeyCode::ArrowUp => {
            update_cursors(
              &mut data.cursors,
              &self.cursors_line_index,
              &self.cursors_line_offset,
              &self.line_spans,
              data.text.text.len(),
              CursorOp::MoveUp,
            );
            ctx.request_paint();
          },
          KeyCode::ArrowDown => {
            update_cursors(
              &mut data.cursors,
              &self.cursors_line_index,
              &self.cursors_line_offset,
              &self.line_spans,
              data.text.text.len(),
              CursorOp::MoveDown,
            );
            ctx.request_paint();
          },
          KeyCode::Home => {
            update_cursors(
              &mut data.cursors,
              &self.cursors_line_index,
              &self.cursors_line_offset,
              &self.line_spans,
              data.text.text.len(),
              CursorOp::MoveBeginLine,
            );
            ctx.request_paint();
          },
          KeyCode::End => {
            update_cursors(
              &mut data.cursors,
              &self.cursors_line_index,
              &self.cursors_line_offset,
              &self.line_spans,
              data.text.text.len(),
              CursorOp::MoveEndLine,
            );
            ctx.request_paint();
          },
          _ => (),
        }

        if let Some(mut inserted_txt) = event.text() {

          // FIXME
          if inserted_txt == "\r" {
            inserted_txt = "\n";
          }

          let ops = data.text.splice(inserted_txt.to_string(), data.cursors.markers[0] as usize);

          data.text = data.text.with(&ops);
          data.cursors = data.cursors.with(&ops);

          ctx.request_paint();
        }
      }
      _ => (),
    }

    for line in &mut self.rows.data {
      line.event(ctx, event, &mut (), env);
    }
  }

  fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &EditorState, env: &Env) {
    for line in &mut self.rows.data {
      line.lifecycle(ctx, event, &(), env);
    }
  }

  fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &EditorState, data: &EditorState, env: &Env) {
    self.update_contents(data);
    self.update_line_spans(data);
    self.update_cursor_pos(data);

    for line in &mut self.rows.data {
      line.update(ctx, &(), env);
    }
  }

  fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &EditorState, env: &Env) -> Size {
    let mut y = 0.0;
    for line in &mut self.rows.data {
      let size = line.layout(ctx, &bc.loosen(), &(), env);
      line.set_layout_rect(ctx, &(), env, Rect::from_origin_size((0.0, y), size));
      y += CHAR_SIZE.1;
    }

    bc.constrain((500.0, 500.0))
  }

  fn paint(&mut self, ctx: &mut PaintCtx, _data: &EditorState, env: &Env) {
    for line in &mut self.rows.data {
      line.paint(ctx, &(), env);
    }

    if self.cursor_is_on {
      for cursor_pos in &self.cursors_px_pos.data {
        ctx.fill(
          Rect::from_origin_size(*cursor_pos, CURSOR_SIZE),
          &Color::rgb8(20, 60, 230),
        );
      }
    }
  }
}
