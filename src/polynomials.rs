use ark_ff::{FftField, Field};
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};

// Silly helper function that index an array of full field evaluations by a random point
pub fn eval_poly<F: FftField>(evals: &[F], r: F) -> F {
    let x_values = vec![F::zero(), F::one()];
    let poly = interpolate_degree_1(&x_values, &evals);

    poly.evaluate(&r)
}

/// Interpolate a degree-1 polynomial given two x-values and two corresponding y-values (evaluations).
/// TODO dumb ChatGpt placeholder until I figure out how to do interpolation with the ark-poly
/// crate
fn interpolate_degree_1<F: FftField>(x_values: &[F], y_values: &[F]) -> DensePolynomial<F> {
    // Compute the Lagrange interpolation for a degree-1 polynomial
    let x0 = x_values[0];
    let x1 = x_values[1];
    let y0 = y_values[0];
    let y1 = y_values[1];

    let term_0_coeff = y0 / (x0 - x1);
    let term_0 = DensePolynomial::from_coefficients_slice(&[-term_0_coeff * x1, term_0_coeff]);

    let term_1_coeff = y1 / (x1 - x0);
    let term_1 = DensePolynomial::from_coefficients_slice(&[-term_1_coeff * x0, term_1_coeff]);

    // The final polynomial is the sum of the two terms
    &term_0 + &term_1
}
