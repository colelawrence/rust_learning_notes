pub trait Measurable {
    fn measured_length(&self) -> f64;
}
impl Measurable for str {
    fn measured_length(&self) -> f64 {
        self.len() as f64
    }
}
impl Measurable for String {
    fn measured_length(&self) -> f64 {
        self.len() as f64
    }
}
impl Measurable for f64 {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for f32 {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for u32 {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for i32 {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for usize {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for isize {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for u64 {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
impl Measurable for i64 {
    fn measured_length(&self) -> f64 {
        self.clone() as f64
    }
}
