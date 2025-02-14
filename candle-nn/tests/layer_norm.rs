#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

use anyhow::Result;
use candle::{Device, Tensor};
use candle_nn::{LayerNorm, Module};

#[test]
fn layer_norm() -> Result<()> {
    let device = &Device::Cpu;
    let w = Tensor::new(&[3f32], device)?;
    let b = Tensor::new(&[0.5f32], device)?;
    let ln = LayerNorm::new(w, b, 1e-8);

    let two = Tensor::new(&[[[2f32]]], device)?;
    let res = ln.forward(&two)?.flatten_all()?;
    assert_eq!(res.to_vec1::<f32>()?, [0.5f32]);

    let inp = Tensor::new(&[[[4f32, 0f32]]], device)?;
    let res = ln.forward(&inp)?;
    assert_eq!(res.to_vec3::<f32>()?, [[[3.5f32, -2.5]]]);

    let inp = Tensor::new(&[[[1f32, 2., 3.], [4., 5., 6.], [9., 8., 7.]]], device)?;
    let res = ln.forward(&inp)?;
    assert_eq!(
        res.to_vec3::<f32>()?,
        [[
            [-3.1742344, 0.5, 4.1742344],
            [-3.1742344, 0.5, 4.1742344],
            [4.1742344, 0.5, -3.1742344]
        ]]
    );
    let mean = (res.sum_keepdim(2)? / 3.0)?;
    // The average value should be `b`.
    assert_eq!(mean.to_vec3::<f32>()?, [[[0.5], [0.5], [0.5]]]);
    let std = (res.broadcast_sub(&mean)?.sqr()?.sum_keepdim(2)?.sqrt()? / 3.0)?;
    // The standard deviation should be sqrt(`w`).
    assert_eq!(
        std.to_vec3::<f32>()?,
        [[[1.7320508], [1.7320508], [1.7320508]]]
    );
    Ok(())
}
