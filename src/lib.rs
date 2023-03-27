use rayon::prelude::*;
use tch::Tensor;

pub fn cpu_load_test() {
 let  slice =  vec! [ 1.0 ;  1000000 ];
 for i in 1..1000000 {
     let  tensor = Tensor::of_slice(&slice).to_device(tch::Device::Cpu);
    println!("{} {:?}", i, tensor);
    }
}


pub fn gpu_load_test() {
    let  slice =  vec! [ 1.0 ;  1000000 ];
    for i in 1..1000000 {
        let  tensor = Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, tensor);
        }
}

pub fn tgpu_load_test() {
    let  slice =  vec! [ 1.0 ;  1000000 ];
    (1..1_000_000).into_par_iter().for_each(|i| {
        let t = Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, t.size())
    });
}
