// 🔐 R1CS 회로 구현
// Groth16 SNARK를 위한 제약조건 시스템

use ark_ff::PrimeField;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};

// ============================================================================
// 1. 간단한 곱셈 회로: x * y = z
// ============================================================================

#[derive(Clone, Debug)]
pub struct MultiplyCircuit<F: PrimeField> {
    pub x: Option<F>,
    pub y: Option<F>,
    pub z: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for MultiplyCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Witness 변수 할당
        let x = cs.new_witness_variable(|| {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let y = cs.new_witness_variable(|| {
            self.y.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let z = cs.new_input_variable(|| {
            self.z.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // 제약조건: x * y = z
        cs.enforce_constraint(
            lc!() + x,
            lc!() + y,
            lc!() + z,
        )?;
        
        Ok(())
    }
}

// ============================================================================
// 2. 비밀번호 해시 회로: secret * 7 + 13 = hash
// ============================================================================

#[derive(Clone, Debug)]
pub struct PasswordCircuit<F: PrimeField> {
    pub secret: Option<F>,
    pub hash: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for PasswordCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Witness: secret (비공개)
        let secret = cs.new_witness_variable(|| {
            self.secret.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Public Input: hash (공개)
        let hash = cs.new_input_variable(|| {
            self.hash.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // 중간 변수: temp = secret * 7
        let temp_value = self.secret.map(|s| s * F::from(7u64));
        let temp = cs.new_witness_variable(|| {
            temp_value.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // 제약조건 1: secret * 7 = temp
        cs.enforce_constraint(
            lc!() + secret,
            lc!() + (F::from(7u64), Variable::One),
            lc!() + temp,
        )?;
        
        // 제약조건 2: temp + 13 = hash
        cs.enforce_constraint(
            lc!() + temp + (F::from(13u64), Variable::One),
            lc!() + Variable::One,
            lc!() + hash,
        )?;
        
        Ok(())
    }
}

// ============================================================================
// 3. 나이 검증 회로: age >= 19
// ============================================================================

#[derive(Clone, Debug)]
pub struct AgeCircuit<F: PrimeField> {
    pub age: Option<F>,
    pub is_adult: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for AgeCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Witness: age (비공개)
        let age = cs.new_witness_variable(|| {
            self.age.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Public Input: is_adult (공개)
        let is_adult = cs.new_input_variable(|| {
            self.is_adult.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // 간단한 검증: age >= 19 → is_adult = 1
        // 실제로는 더 복잡한 비교 회로 필요
        // 여기서는 데모용으로 단순화
        
        // age - 19 >= 0 검증 (간소화)
        let diff_value = self.age.map(|a| a - F::from(19u64));
        let diff = cs.new_witness_variable(|| {
            diff_value.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // 제약조건: age = diff + 19
        cs.enforce_constraint(
            lc!() + diff + (F::from(19u64), Variable::One),
            lc!() + Variable::One,
            lc!() + age,
        )?;
        
        // is_adult = 1 (성인)
        cs.enforce_constraint(
            lc!() + is_adult,
            lc!() + Variable::One,
            lc!() + Variable::One,
        )?;
        
        Ok(())
    }
}

// ============================================================================
// 4. 제곱 회로: x² = y
// ============================================================================

#[derive(Clone, Debug)]
pub struct SquareCircuit<F: PrimeField> {
    pub x: Option<F>,
    pub y: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for SquareCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Witness: x (비공개)
        let x = cs.new_witness_variable(|| {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Public Input: y (공개)
        let y = cs.new_input_variable(|| {
            self.y.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // 제약조건: x * x = y
        cs.enforce_constraint(
            lc!() + x,
            lc!() + x,
            lc!() + y,
        )?;
        
        Ok(())
    }
}

// ============================================================================
// 5. 헬퍼 함수들
// ============================================================================

/// 비밀번호 해시 계산
pub fn compute_password_hash<F: PrimeField>(secret: F) -> F {
    secret * F::from(7u64) + F::from(13u64)
}

/// 나이 검증
pub fn is_adult<F: PrimeField>(age: F) -> F {
    if age >= F::from(19u64) {
        F::from(1u64)
    } else {
        F::from(0u64)
    }
}

/// 제곱 계산
pub fn compute_square<F: PrimeField>(x: F) -> F {
    x * x
}

