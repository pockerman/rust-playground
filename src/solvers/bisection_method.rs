use super::solver_result::{SolverResult, SolverType};

///
/// Bisection method. The bisection is easy to understand
/// assumming we want a solution to the equation f(x) = 0
/// then we choose an interval [x1,x2] such that 
/// f(x1)f(x2) < 0 i.e f changes signs in this interval
///
/// 
///

pub struct BisectionSolver
{
	pub x1: f64,
	pub x2: f64,
	pub itrs: usize,
}

impl BisectionSolver{
	
	
	pub fn new(x1: f64, x2: f64, itrs: usize) -> Self{
		BisectionSolver{x1:x1, x2:x2, itrs: itrs}
	}
	
	pub fn solve(&self, f: fn(f64) -> f64) -> Result<SolverResult, String>{
		
		
		// make sure we change sign
		if f(self.x1) * f(self.x2) > 0.0 {
			let msg = format!("The given function does not seem to change signs in [{}, {}]", self.x1, self.x2);
			return Err(msg);
		}
		
		
		
		let mut x_l = self.x1;
		let mut x_u = self.x2;
		
		
		for itr in 0..self.itrs{
			
			// find an estimate of the root
			let mut x_r = ( x_l + x_u ) * 0.5;
			
			// the solution is x_r
			if f(x_l) * f(x_r) == 0.0 {
				return Ok(SolverResult::new(x_r, 0.0, 
				                            itr, true,
											SolverType::BisectionSolverType));
			}
			
			// the root lies in the lower 
			// subinterval
			if f(x_l) * f(x_r) < 0.0{
				
				x_u = x_r;
			}
			else{
				
				// the root lies in the upper subinterval
				x_l = x_r;
			}
		}
		
		return Err(format!("No solution was found in the interval [{}, {}] for {} iterations", 
		                   self.x1, self.x2, self.itrs));
	}
	
}


#[cfg(test)]
mod tests{
	
	use super::*;
	
	fn f(x: f64) -> f64{
		x
	}
	
	#[test]
	fn err_when_sign_does_not_change(){
		
		let mut method = BisectionSolver::new(1.0, 2.0, 10);
		let result = method.solve(f);
		
		assert_eq!(result.is_err(), true);
	}
	
	#[test]
	fn err_when_iterations_not_enough(){
		
		// zero iterations
		let mut method = BisectionSolver::new(1.0, -2.0, 0);
		let result = method.solve(f);
		
		assert_eq!(result.is_err(), true);
	}
	
	#[test]
	fn ok_when_solution_exists(){
		
		// zero iterations
		let mut method = BisectionSolver::new(-1.0, 1.0, 10);
		let result = method.solve(f);
		
		assert_eq!(result.is_ok(), true);
		
		let result = result.unwrap();
		assert_eq!(result.solution, 0.0);
	}
}