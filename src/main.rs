mod integrators;
use integrators::euler::Euler;

mod solvers;
use solvers::FixedPointIteration;
use solvers::SolverResult;

mod mesh;
use mesh::Point2D;


use libm;


fn exponential(x: f64) -> f64{
	libm::exp(-x)
}

fn main() {
    
	/*println!("Creating a new Point2D");
	
	// create the origin
	let p0 = Point2D::new(0.0, 0.0, 0);
	
	println!("x: {}", p0.x);
	println!("y: {}", p0.y);
	println!("idx: {}", p0.idx);
	
	// create the origin
	let p1 = Point2D::new(1.0, 1.0, 1);
	
	println!("x: {}", p1.x);
	println!("y: {}", p1.y);
	println!("idx: {}", p1.idx);
	
	// mut becuase we change the init value
	let mut euler: Euler = Euler::new(0.0, 0.01);
	let position = euler.integrate(0.01);
	println!("New position {}", position);
	
	let position = euler.integrate(0.02);
	println!("New position {}", position);
	*/
	
	
	let solver = FixedPointIteration::new(20, 0.0001);
	let result = solver.solve(0.0, exponential);
	
	println!("Solver result {}", result);
	
	
	
}
