///
/// Utility used to model a function to be used
/// by the various solvers.
///
pub trait FunctionBase
{
	
	///
	/// Returns the value of the function at the given 
	/// point
	///
	fn f(&self, x: f64) -> f64;
	
	
	///
	/// Returns the first derivative of the function
	/// at the given point
	///
	fn df(&self, x: f64) -> f64;
	
}