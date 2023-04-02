use super::{FloatParam, ReverbParameters};

pub struct ParamIndexer<'a> {
  pub inner: &'a ReverbParameters,
}

impl<'a> ParamIndexer<'a> {
  fn get_param(&self, index: i32) -> Option<&'a FloatParam> {
    let param = match index {
      0 => &self.inner.predelay,
      1 => &self.inner.size,
      2 => &self.inner.speed,
      3 => &self.inner.depth,
      4 => &self.inner.shimmer,
      5 => &self.inner.absorb,
      6 => &self.inner.decay,
      7 => &self.inner.tilt,
      8 => &self.inner.mix,
      _ => return None,
    };
    Some(param)
  }
}

impl ReverbParameters {
  pub fn get_param_by_index(&self, index: i32) -> Option<&FloatParam> {
    let param_indexer = ParamIndexer { inner: self };
    param_indexer.get_param(index)
  }
}
