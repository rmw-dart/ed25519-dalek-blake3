use digest::generic_array::{typenum::U64, GenericArray};
use digest::{Reset, Update};

#[derive(Clone)]
pub struct Hasher {
  h: blake3::Hasher,
}

impl Update for Hasher {
  #[inline]
  fn update(&mut self, data: impl AsRef<[u8]>) {
    self.h.update(data.as_ref());
  }
}

impl Reset for Hasher {
  #[inline]
  fn reset(&mut self) {
    self.h.reset();
  }
}

impl Default for Hasher {
  #[inline]
  fn default() -> Self {
    Hasher {
      h: blake3::Hasher::new(),
    }
  }
}

impl digest::FixedOutput for Hasher {
  type OutputSize = U64;

  #[inline]
  fn finalize_into(self, out: &mut GenericArray<u8, Self::OutputSize>) {
    self.h.finalize_xof().fill(out);
  }

  #[inline]
  fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
    self.h.finalize_xof().fill(out);
    self.h.reset();
  }
}
