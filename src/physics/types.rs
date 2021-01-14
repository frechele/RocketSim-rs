use std::ops::{Add, Sub};

pub struct Vector2D {
	pub x: f32,
	pub y: f32
}

impl Vector2D {
	pub fn create(length: f32, radian: f32) -> Self {
		Self {
			x: length * radian.cos(),
			y: length * radian.sin()
		}
	}

	pub fn length(&self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn multiply(&self, v: f32) -> Self {
		Self {
			x: self.x * v,
			y: self.y * v
		}
	}

	pub fn normalize(self) -> Self {
		let length = self.length();

		self.multiply(1. / length)
	}
}

impl Add for Vector2D {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y
		}
	}
}

impl Sub for Vector2D {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y
		}
	}
}
