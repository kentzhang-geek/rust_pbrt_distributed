mod distribution;
use ddsfile;
use ddsfile::{AlphaMode, D3D10ResourceDimension, NewD3dParams};
use distribution::*;

fn main() {
    // config for it
    let sample_num = 32;
    let sequance_num = 128;
    // generate 2d distribution and save to dds texture
    let mut ddsfile = ddsfile::Dds::new_dxgi({ddsfile::NewDxgiParams{
        width: sample_num as u32,
        height: sequance_num as u32,
        format: ddsfile::DxgiFormat::R32G32_Float,
        depth: Some(1),
        mipmap_levels: Some(1),
        array_layers: Some(1),
        caps2: None,
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode: AlphaMode::Opaque,
    }}).unwrap();
    let mut data_pointer = ddsfile.get_mut_data(0).unwrap().as_mut_ptr() as * mut (f32,f32);
    for i in 0..sequance_num {
        let res = distribution::StratifiedDistribution2DImpl::new().sample(sample_num, sample_num);
        for j in 0..sample_num {
            let u = res[j].0;
            let v = res[j].1;
            let idx = (i * sample_num + j);
            unsafe { data_pointer.wrapping_add(idx).write((u, v)); }
        }
    }
    let mut writefile = std::fs::File::create("test.dds").unwrap();
    ddsfile.write(&mut writefile).unwrap();
}