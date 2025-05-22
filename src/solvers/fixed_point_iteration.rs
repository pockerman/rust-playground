
use super::solver_result::{SolverResult, SolverType};

///
/// Fixed point iteration solver
///

pub struct FixedPointIteration{
	
	/// number of iterations to perform
	itrs: usize,
	
	/// tolerace that indicates the solution 
	/// is acceptable
	tol: f64,
}


impl FixedPointIteration {
	
	pub fn new(itrs: usize, tol: f64) -> Self{
		FixedPointIteration{itrs:itrs, tol:tol}
	}
	
	
	pub fn solve(&self, x_init: f64, rhs: fn(f64) -> f64) -> SolverResult{
		
		let mut x_old = x_init;
		let mut error = 0.0;
		
		for itr in 0..self.itrs {
			
			// compute point to evaluate
			let x_new = rhs(x_old);
			
			// compute error
			error = self.estimate_error(x_new, x_old);
			
			println!("Iteration {} error estimate {}", itr, error);
			
			// check the error against the tolerance
			if error < self.tol{
				
				// we have converged.
				// Return the solution the number 
				// of iterations, the error and the olerance
				println!("Solution converged in {} iterations and error {}", itr, error);
				
				return SolverResult::new(x_new, error, itr, 
										 true,
				                         SolverType::FixedPointSolverType);
			}
			
			// update the old variable and continue
			x_old = x_new;
			
		}
		
		SolverResult::new(x_old, error, 
		                  self.itrs, false,
		                  SolverType::FixedPointSolverType)
		
	}
	
	fn estimate_error(&self, x_new: f64, x_old: f64) -> f64{
		
		let error = (x_new - x_old) / x_new;
		return error.abs();
	}
}