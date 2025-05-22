use std::fmt;

///
/// Utility enum to flag the various solvers
/// Each solver is responsible for tagging
/// The SolverResult object it returns from
/// solve function.
///
pub enum SolverType{
	
	BisectionSolverType,
	FixedPointSolverType,
	NewtonRaphsonSolverType,
	
}


///
/// Utility struct to hold the result
/// of solver in a uniform manner. 
/// It implements the Display trait 
///
pub struct SolverResult{
	
	pub solution: f64,
	pub error: f64,
	pub itrs: usize,
	pub converged: bool,
	pub solver_type: SolverType
	
}


impl SolverResult {
	
	pub fn new(solution: f64, error: f64, itrs: usize, 
			   converged: bool, solver_type: SolverType) -> Self{
		SolverResult{solution: solution, 
		             error: error, 
					 itrs:itrs, 
					 converged:converged,
					 solver_type: solver_type}
	}
}


impl fmt::Display for SolverResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "solution:  {}, error: {}, itrs: {}, converged: {}", 
		       self.solution, self.error, self.itrs, self.converged)
    }
}