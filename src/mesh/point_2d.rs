///
/// Models a Point in 2-d space. A Node2D has
/// a position and an id. The position and the id of
/// the node are typically set by the various mesh generators
///

pub struct Point2D{
	
	x: f64,
	y: f64,
	
}

impl Point2D{
	
	
	pub fn new(x: f64, y: f64) -> Point2D{
		Point2D{x:x, y:y}
	}
	
	///
	/// Get the x coordinate of the point
	///
	pub fn x(&self) -> f64{
		self.x
	}
	
	
	///
	/// Get the y coordinate of the point
	///
	pub fn y(&self) -> f64{
		self.y
	}
	
}

#[cfg(test)]
mod tests{
	
	use super::*;
	

	#[test]
	fn create_point2d_correctly(){
		
		let p = Point2D::new(0.0, 0.0);
		let x = p.x();
		let y = p.y();
		
		assert_eq!(x, 0.0);
		assert_eq!(y, 0.0);
	}
	
}
