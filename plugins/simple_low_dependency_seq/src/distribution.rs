use rand::Rng;

/// Interface for 1D distribution
pub trait Distribution1D {
    /// generate uniform 1D distribution, low dependency sequence
    fn sample(&self, n: usize) ->Vec<f32>;
    fn new() -> Box<dyn Distribution1D> where Self: Sized;
}

/// Interface for 2D distribution
pub trait Distribution2D {
    /// generate uniform 2D distribution, low dependency sequence
    fn sample(&self, nx: usize, ny:usize) ->Vec<(f32, f32)>;
    fn new() -> Box<dyn Distribution2D> where Self: Sized;
}

pub struct StratifiedDistribution1DImpl {
}

impl Distribution1D for StratifiedDistribution1DImpl {
    fn sample(&self, n: usize) -> Vec<f32> {
        let mut result = Vec::new();
        for i in 0..n as usize {
            result.push((i as f32 + 0.5f32) / (n as f32));
        }
        // now jitter
        let mut rng = rand::thread_rng();
        for i in 0..n as usize {
            let u = rng.gen::<f32>().min(1.0f32 - f32::EPSILON).max(f32::EPSILON);
            result[i] = result[i] + (u - 0.5f32) / (n as f32);
        }
        // shuffle
        let mut shuf_rng = rand::thread_rng();
        for i in 0..n as usize {
            let j = rng.gen_range(i..n as usize);
            let tmp = result[i];
            result[i] = result[j];
            result[j] = tmp;
        }
        return result;
    }

    fn new() -> Box<dyn Distribution1D> {
        return Box::new(StratifiedDistribution1DImpl{});
    }
}

pub struct StratifiedDistribution2DImpl {
}

impl Distribution2D for StratifiedDistribution2DImpl {
    fn sample(&self, nx: usize, ny:usize) -> Vec<(f32, f32)> {
        let mut result = Vec::new();
        for i in 0..nx {
            for j in 0..ny {
                let u = (i as f32 + 0.5f32) / (nx as f32);
                let v = (j as f32 + 0.5f32) / (nx as f32);
                result.push((u, v));
            }
        }
        let mut rng_x = rand::thread_rng();
        let mut rng_y = rand::thread_rng();
        // jitter
        for i in 0..result.len() {
            let mut u = rng_x.gen::<f32>().min(1.0f32 - f32::EPSILON).max(f32::EPSILON);
            let mut v = rng_y.gen::<f32>().min(1.0f32 - f32::EPSILON).max(f32::EPSILON);
            u = result[i].0 + (u - 0.5f32) / (nx as f32);
            v = result[i].1 + (v - 0.5f32) / (ny as f32);
            result[i] = (u, v);
        }
        // shuffle
        let mut shuf_rng = rand::thread_rng();
        for i in 0..result.len() {
            let j = shuf_rng.gen_range(i..result.len());
            let tmp = result[i];
            result[i] = result[j];
            result[j] = tmp;
        }
        return result;
    }

    fn new() -> Box<dyn Distribution2D> {
        return Box::new(StratifiedDistribution2DImpl{});
    }
}

/// test for StratifiedDistribution1DImpl
#[cfg(test)]
mod tests {
    use super::*;
    use plotters::prelude::*;

    pub fn scatter1D(results : & Vec<f32>) {
        let s_size = results.len();
        let root = BitMapBackend::new("test1D.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Scatter Random", ("sans-serif", 40))
            .build_cartesian_2d(0..s_size, 0f32..1f32)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(
            (0..s_size).map(|idx| TriangleMarker::new((idx, results[idx]), 5, &BLUE)),
        )
            .unwrap();

        root.present().unwrap();
    }

    pub fn scatter2D(results : & Vec<(f32, f32)>) {
        let s_size = results.len();
        let root = BitMapBackend::new("test2D.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut ctx = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Scatter Random", ("sans-serif", 40))
            .build_cartesian_2d(0f32..1f32, 0f32..1f32)
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series(
            (0..s_size).map(|idx| TriangleMarker::new(results[idx], 5, &BLUE)),
        )
            .unwrap();

        root.present().unwrap();
    }

    #[test]
    fn it_works() {
        let s_size = 128;
        let result = StratifiedDistribution1DImpl::new().sample(s_size);
        scatter1D(&result);
        let size2d = 16;
        let result2d = StratifiedDistribution2DImpl::new().sample(size2d, size2d);
        println!("{:?}", result2d);
        scatter2D(&result2d);
    }
}