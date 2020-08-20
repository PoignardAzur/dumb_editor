use im::Vector;
use druid::{Data, Lens};
use crate::rope::{Rope, RopeMarkers, RopeSpans, RopeData, RopeOps};

pub enum CursorOp {
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,
  MoveBeginLine,
  MoveEndLine,
  MovePrevWord,
  MoveNextWord,
}

// TODO
// We assume that lines end with "\n" and not eg "\r\n"
const END_LINE_CHARACTERS: usize = 1;

pub fn update_cursors(
  cursors: &mut RopeMarkers,
  cursors_line_index: &RopeData<usize>,
  cursors_line_offset: &RopeData<usize>,
  line_spans: &RopeSpans,
  text_size: usize,
  op: CursorOp
) {
  assert!(cursors.markers.len() == cursors_line_index.data.len());
  assert!(cursors.markers.len() == cursors_line_offset.data.len());

  let mut cursors_line_index_iter = cursors_line_index.data.iter();
  let mut cursors_line_offset_iter = cursors_line_offset.data.iter();
  for cursor in &mut cursors.markers.iter_mut() {
    let line_index = *cursors_line_index_iter.next().unwrap();
    let line_offset = *cursors_line_offset_iter.next().unwrap();

    dbg!(&line_spans.spans);
    dbg!(line_index);
    dbg!(line_offset);
    let line_span = *line_spans.spans.get(line_index).unwrap();

    let prev_line_span = if line_index > 0 { line_spans.spans.get(line_index - 1) } else { None };
    let next_line_span = line_spans.spans.get(line_index + 1);

    match op {
      CursorOp::MoveLeft => {
        *cursor = cursor.saturating_sub(1);
      },
      CursorOp::MoveRight => {
        if *cursor < text_size {
          *cursor += 1;
        }
      },
      CursorOp::MoveUp => {
        // TODO - Handle hard tabs and unicode characters
        if let Some(&prev_line_span) = prev_line_span {
          *cursor -= std::cmp::max(line_offset + END_LINE_CHARACTERS, prev_line_span);
        }
        else {
          *cursor = 0;
        }
      },
      CursorOp::MoveDown => {
        // TODO - Handle hard tabs and unicode characters
        if let Some(&next_line_span) = next_line_span {
          assert!(next_line_span >= END_LINE_CHARACTERS);
          *cursor -= line_offset;
          *cursor += std::cmp::min(line_offset, next_line_span - END_LINE_CHARACTERS);
          *cursor += line_span;
        }
        else {
          *cursor = text_size;
        }
      },
      CursorOp::MoveBeginLine => {
        *cursor -= line_offset;
      },
      CursorOp::MoveEndLine => {
        assert!(line_span >= END_LINE_CHARACTERS);
        // We're stopping right before the '\n'
        *cursor = *cursor - line_offset + line_span - END_LINE_CHARACTERS;
      },
      CursorOp::MovePrevWord => {
        todo!();
      },
      CursorOp::MoveNextWord => {
        todo!();
      },
    }
  }

  // TODO - merge cursors
}
