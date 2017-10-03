
struct Curve {
  len: usize,
  xs: Vec<f32>,
}
impl Curve {
  pub fn new(len: usize, xs: &[f32]) -> Curve {
    assert!(len == xs.len());
    Curve { len: len, xs: xs.clone() }
  }
  pub fn interpolate(&self, t: f32) -> f32 {
    let mut xs = self.xs.clone();
    self.inter(t, self.len, &mut xs)
  }
  fn inter(&self, t: f32, len: usize, mut arr: &[f32]) -> f32 {
    if len == 2 {
      return arr[0];
    }
    for i in 0..(len-1) {
      println!("{}", i); // x: i32
    }
    0.0
  }
}

pub fn test() {
  let xs = [7.0, 9.0, 11.0];
  Curve::new(3, &xs).interpolate(0.5);
}
