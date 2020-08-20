use im::Vector;
use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct Rope {
  pub text: String,
}

#[derive(Clone, Data, Lens)]
pub struct RopeMarkers {
  pub markers: Vector<usize>,
}

#[derive(Clone, Data, Lens)]
pub struct RopeSpans {
  pub spans: Vector<usize>,
}

pub struct RopeData<T> {
  pub data: Vec<T>,
}

pub struct RopeOps {
  pub ops: Vec<(usize, usize, String)>,
}


impl Rope {
  pub fn splice(&self, str: String, pos: usize) -> RopeOps {
    RopeOps { ops: vec![
      (pos, pos, str),
    ] }
  }

  pub fn with(&mut self, ops: &RopeOps) -> Self {
    let mut new_rope = self.clone();
    let mut gap = 0;

    for op in &ops.ops {
      new_rope.text.replace_range((gap + op.0)..(gap + op.1), &op.2);
      gap -= op.1 - op.0;
      gap += op.2.len();
    }

    new_rope
  }
}


impl RopeMarkers {
  pub fn with(&mut self, ops: &RopeOps) -> Self {
    let mut new_markers = self.clone();
    let mut gap = 0;

    // FIXME - This assumes that ops are sorted
    for op in &ops.ops {
      for marker in new_markers.markers.iter_mut() {
        if *marker >= op.0 {
          let removed_space = std::cmp::min(op.1, *marker) - op.0;
          *marker += gap + op.2.len() - removed_space;
        }
      }

      gap -= op.1 - op.0;
      gap += op.2.len();
    }

    new_markers
  }
}

impl RopeSpans {
  pub fn with(&mut self, ops: &RopeOps) -> Self {
    let mut new_spans = self.clone();

    // FIXME - This doesn't address the case where adding text splits / merges spans

    // FIXME - This assumes that ops are sorted
    // TODO - iterator.take_until
    for op in &ops.ops {
      let mut acc_length = 0;
      for span in new_spans.spans.iter_mut() {
        // FIXME
        if op.0 >= acc_length && op.0 < acc_length + *span {
          *span += op.1 - op.0;
          *span -= op.2.len();
        }

        acc_length += *span;
      }
    }

    new_spans
  }
}

pub struct DumbSpanMatch {
  pub index: usize,
  pub match_gap: usize,
}

pub fn match_spans(markers: &RopeMarkers, spans: &RopeSpans) -> RopeData<DumbSpanMatch> {
  let span_matches = markers.markers.iter()
    .map(|marker| {
      let mut i = 0;
      let mut acc_length = 0;
      for span in &spans.spans {
        if *marker >= acc_length && *marker < acc_length + *span {
          return DumbSpanMatch { index: i, match_gap: *marker - acc_length };
        }
        i += 1;
        acc_length += *span;
      }
      dbg!(*marker);
      dbg!(&spans.spans);
      panic!("cannot match marker to span");
    })
    .collect();

  RopeData { data: span_matches }
}
