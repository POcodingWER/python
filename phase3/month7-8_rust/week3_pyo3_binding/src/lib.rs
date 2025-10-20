// PyO3: Rust → Python 바인딩
// Python에서 Rust 함수를 호출할 수 있게 만들기

use pyo3::prelude::*;

/// Linear Regression 모델 (Rust 구현)
#[pyclass]
struct LinearModel {
    weight: f64,
    bias: f64,
}

#[pymethods]
impl LinearModel {
    /// 생성자
    #[new]
    fn new(weight: f64, bias: f64) -> Self {
        LinearModel { weight, bias }
    }

    /// 단일 예측
    fn predict(&self, x: f64) -> f64 {
        self.weight * x + self.bias
    }

    /// 배치 예측
    fn predict_batch(&self, inputs: Vec<f64>) -> Vec<f64> {
        inputs.iter().map(|&x| self.predict(x)).collect()
    }

    /// 모델 정보 출력
    fn __repr__(&self) -> String {
        format!(
            "🦀rust:LinearModel(weight={}, bias={})",
            self.weight, self.bias
        )
    }
}

/// 간단한 덧셈 함수 (예제)
#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    println!("🦀rust: add function called with a={}, b={}", a, b);
    a + b
}

/// 벡터 내적 (성능 비교용)
#[pyfunction]
fn dot_product(a: Vec<f64>, b: Vec<f64>) -> PyResult<f64> {
    if a.len() != b.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "벡터 길이가 다릅니다",
        ));
    }

    let result: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    Ok(result)
}

/// Python 모듈 정의
#[pymodule]
fn zkml_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinearModel>()?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(dot_product, m)?)?;
    Ok(())
}
