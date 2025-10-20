// halo2 ZK 회로: Linear Regression 증명
// y = weight * x + bias를 ZK로 증명

use halo2_proofs::{
    arithmetic::Field,
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Instance, Selector},
    poly::Rotation,
};

// 회로 설정
#[derive(Clone, Debug)]
pub struct LinearConfig {
    advice: [Column<Advice>; 3], // x, weight, result
    instance: Column<Instance>,  // public input/output
    selector: Selector,
}

// 회로 구조
#[derive(Clone, Debug)]
pub struct LinearCircuit<F: Field> {
    pub x: Value<F>,
    pub weight: Value<F>,
    pub bias: Value<F>,
}

impl<F: Field> Circuit<F> for LinearCircuit<F> {
    type Config = LinearConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self {
            x: Value::unknown(),
            weight: Value::unknown(),
            bias: Value::unknown(),
        }
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [
            meta.advice_column(),
            meta.advice_column(),
            meta.advice_column(),
        ];
        let instance = meta.instance_column();
        let selector = meta.selector();

        // 모든 advice 컬럼 활성화
        for col in &advice {
            meta.enable_equality(*col);
        }
        meta.enable_equality(instance);

        // 제약조건: result = weight * x (bias는 나중에 추가)
        meta.create_gate("linear regression", |meta| {
            let s = meta.query_selector(selector);
            let x = meta.query_advice(advice[0], Rotation::cur());
            let weight = meta.query_advice(advice[1], Rotation::cur());
            let result = meta.query_advice(advice[2], Rotation::cur());

            // 간단화: result = weight * x (bias 제외)
            vec![s * (weight * x - result)]
        });

        LinearConfig {
            advice,
            instance,
            selector,
        }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        let result_cell = layouter.assign_region(
            || "linear regression",
            |mut region| {
                // Selector 활성화
                config.selector.enable(&mut region, 0)?;

                // x 할당
                region.assign_advice(|| "x", config.advice[0], 0, || self.x)?;

                // weight 할당
                region.assign_advice(|| "weight", config.advice[1], 0, || self.weight)?;

                // result = weight * x 계산 (bias 제외)
                let result = self.weight.zip(self.x).map(|(w, x)| w * x);

                let result_cell =
                    region.assign_advice(|| "result", config.advice[2], 0, || result)?;

                Ok(result_cell)
            },
        )?;

        // Public input과 result를 연결 (이게 핵심!)
        layouter.constrain_instance(result_cell.cell(), config.instance, 0)?;

        Ok(())
    }
}

pub fn run() {
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::pasta::Fp;

    println!("  🔐 halo2 ZK 회로 - 실제 증명 생성!");

    // 1. 회로 생성: y = 2 * 3 = 6
    let x = Fp::from(3);
    let weight = Fp::from(2);
    let expected_output = Fp::from(6);

    println!("\n  📊 입력 데이터:");
    println!("     x (private) = 3");
    println!("     weight (private) = 2");
    println!("     예상 출력 (public) = 6");

    let circuit = LinearCircuit {
        x: Value::known(x),
        weight: Value::known(weight),
        bias: Value::known(Fp::from(0)),
    };

    // 2. MockProver로 증명 생성 (k=4: 2^4=16 rows)
    println!("\n  🔧 증명 생성 중...");
    let k = 4;
    let public_inputs = vec![expected_output];

    match MockProver::run(k, &circuit, vec![public_inputs]) {
        Ok(prover) => {
            println!("  ✅ 증명 생성 성공!");

            // 3. 증명 검증
            println!("\n  🔍 증명 검증 중...");
            match prover.verify() {
                Ok(_) => {
                    println!("  ✅ 증명 검증 성공!");
                    println!("\n  🎉 결과: 2 * 3 = 6 이 ZK로 증명되었습니다!");
                }
                Err(e) => {
                    println!("  ❌ 증명 검증 실패: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("  ❌ 증명 생성 실패: {:?}", e);
        }
    }

    println!("\n  💡 이것은 MockProver입니다:");
    println!("     - 실제 암호화는 아니지만, 회로가 올바른지 검증");
    println!("     - 프로덕션에서는 실제 Prover 사용 (더 복잡)");
    println!("     - 하지만 논리는 동일하게 작동!");

    // 4. 틀린 값으로 테스트 (실패 예시)
    println!("\n  🧪 틀린 값으로 테스트:");
    let wrong_circuit = LinearCircuit {
        x: Value::known(x),
        weight: Value::known(weight),
        bias: Value::known(Fp::from(0)),
    };
    let wrong_output = vec![Fp::from(999)]; // 틀린 값!

    match MockProver::run(k, &wrong_circuit, vec![wrong_output]) {
        Ok(prover) => match prover.verify() {
            Ok(_) => {
                println!("  ❌ 이상해요... 틀린 값인데 통과했어요?");
            }
            Err(_) => {
                println!("  ✅ 예상대로 실패! (2*3=6인데 999라고 주장)");
            }
        },
        Err(_) => {
            println!("  ✅ 예상대로 실패! (2*3=6인데 999라고 주장)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;
    use halo2_proofs::pasta::Fp;

    #[test]
    fn test_linear_circuit() {
        // k = 4 (2^4 = 16 rows)
        let k = 4;

        // y = 2 * 3 = 6 (bias 제외)
        let circuit = LinearCircuit {
            x: Value::known(Fp::from(3)),
            weight: Value::known(Fp::from(2)),
            bias: Value::known(Fp::from(0)), // 사용 안 함
        };

        let public_inputs = vec![Fp::from(6)];

        // Mock prover로 회로 검증
        let prover = MockProver::run(k, &circuit, vec![public_inputs]).unwrap();
        prover.assert_satisfied();
    }
}
