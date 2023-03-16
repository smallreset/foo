pub struct Foo {
    _PI: f32,
    pub a: u32,
    pub b: u32,
}

impl Foo {
    pub fn new(a: u32, b: u32) -> Self {
        Foo {
            _PI: 3.1415926,
            a,
            b
        }
    }

    pub fn default() -> Self {
        Foo {
            _PI: 3.1415926,
            a: 0,
            b: 0
        }
    }

    pub fn PI(&self) -> f32 {
        self._PI
    }

    pub fn set_PI(&mut self, value: f32) {
        self._PI = value;
    }

    pub fn add(&self) -> i64 {
        self.a as i64 + self.b as i64
    }

    pub fn sub(&self) -> i64 {
        self.a as i64 - self.b as i64
    }

    pub fn inc_a(&mut self) {
        self.a += 1;
    }

}
