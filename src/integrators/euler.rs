/// Euler integration
//pub mod euler{
//	
//	pub fn forward(){
//		println!("Euler integration...");
//	}
//}


pub struct Euler{
	
	// the time step to use
	dt: f64,
	
	// the initial condition
	// it is updated every time 
	// integrate is called
	x_0: f64,
}

impl Euler {
	
	pub fn new(dt: f64, x0: f64) -> Self{
		Euler{dt:dt, x_0: x0}
	}
	
	pub fn integrate(&mut self, f: f64) -> f64{
		let x: f64 = f * self.dt - self.x_0;
		self.x_0 = x;
		x
	}
}

