use super::solver_result::{SolverResult, SolverType};
use super::function_base::FunctionBase;

///
/// Fixed point iteration solver
///

pub struct NewtonRaphsonSolver{
	
	/// number of iterations to perform
	itrs: usize,
	
	/// tolerace that indicates the solution 
	/// is acceptable
	tol: f64,
}


impl NewtonRaphsonSolver {
	
	pub fn new(itrs: usize, tol: f64) -> Self{
		NewtonRaphsonSolver{itrs:itrs, tol:tol}
	}
	
	
	pub fn solve(&self, x_init: f64, func: &dyn FunctionBase) -> SolverResult{
		
		let mut x_old = func.f(x_init);
		let mut x_old_d = func.df(x_init);
		let mut error = 0.0;
		
		SolverResult::new(x_old, error, 
		                  self.itrs, false,
		                  SolverType::NewtonRaphsonSolverType)
		
	}
}


#[cfg(test)]
mod tests{
	
	use super::*;
	
	fn f(x: f64) -> f64{
		x
	}
	
	/*#[test]
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
	}*/
}