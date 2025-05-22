
use super::point_2d::Point2D;


///
/// Models a Node in 2-d space. A Node2D has
/// a position and an id. The position and the id of
/// the node are typically set by the various mesh generators
///

pub struct Node2D{
	
	pos: Point2D,
	idx: usize,
}


impl Node2D{
	
	pub fn new(p: Point2D, idx: usize) -> Node2D{
		Node2D{pos:p, idx:idx}
	}
	
	///
	/// Get the global id of the node
	///
	pub fn get_idx(&self) -> usize{
		self.idx
	}
	
	///
	/// Get the global position of the node
	///
	pub fn get_pos(&self) -> &Point2D{
		&self.pos
	}
}



#[cfg(test)]
mod tests{
	
	use super::*;
	
	
	
	#[test]
	fn create_node_correctly(){
		
		let node = Node2D::new(Point2D::new(0.0, 0.0), 10);
		let idx = node.get_idx();
		
		assert_eq!(idx, 10);
	}
	
	
	
	
}

