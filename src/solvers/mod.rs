pub mod solver_result;
pub mod fixed_point_iteration;
pub mod bisection_method;
pub mod newton_raphson;
pub mod function_base;

pub use solver_result::{SolverResult, SolverType};
pub use fixed_point_iteration::FixedPointIteration;
pub use bisection_method::BisectionSolver;
pub use newton_raphson::NewtonRaphsonSolver;