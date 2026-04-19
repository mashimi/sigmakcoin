use p3_field::PrimeField;
use p3_air::{BaseAir, Air};

pub struct GradientCircuit {
    pub loss_threshold: u32,
}

impl GradientCircuit {
    pub fn new(loss_threshold: u32) -> Self {
        Self { loss_threshold }
    }
}

impl<F: PrimeField> BaseAir<F> for GradientCircuit {
    fn width(&self) -> usize {
        5 // loss_before, loss_after, gradient_norm, step_counter, is_valid
    }
}

impl<F: PrimeField> Air<F> for GradientCircuit {
    fn eval(&self, builder: &mut impl p3_air::AirBuilder<F>) {
        let loss_before = builder.get_local(0);
        let loss_after = builder.get_local(1);
        let gradient_norm = builder.get_local(2);
        let step_counter = builder.get_local(3);
        let is_valid = builder.get_local(4);

        // Check that loss actually decreased
        let loss_decrease = loss_before - loss_after;
        builder.assert_ge(loss_decrease, F::from_canonical_checked(self.loss_threshold));

        // Check that gradient norm is reasonable (not too large)
        builder.assert_lt(gradient_norm, F::from_canonical_checked(1000));

        // Check that step counter is reasonable
        builder.assert_lt(step_counter, F::from_canonical_checked(10000));

        // Check that is_valid is boolean
        builder.assert_bool(is_valid);
    }
}

pub fn generate_gradient_proof(loss_before: f32, loss_after: f32, gradient_norm: f32, steps: usize) -> Vec<u8> {
    // Simplified proof generation - in a real implementation this would use Plonky3
    let proof_data = format!(
        "gradient_proof:{},{},{},{}",
        loss_before, loss_after, gradient_norm, steps
    );
    proof_data.into_bytes()
}

pub fn verify_gradient_proof(proof: &[u8], threshold: u32) -> Result<bool, Box<dyn std::error::Error>> {
    let proof_str = std::str::from_utf8(proof)?;
    if proof_str.starts_with("gradient_proof:") {
        let parts: Vec<&str> = proof_str[15..].split(',').collect();
        if parts.len() == 4 {
            let loss_before: f32 = parts[0].parse()?;
            let loss_after: f32 = parts[1].parse()?;
            let loss_decrease = loss_before - loss_after;
            
            return Ok(loss_decrease >= threshold as f32);
        }
    }
    Ok(false)
}