unsafe extern "C" {
    fn cuda_add_kernel(a: *mut f32, b: *mut f32, c: *mut f32, n: i32);
    fn cudaMallocGeneric(data: *mut *mut std::ffi::c_void, size: usize) -> i32;
    fn cudaMemcpyGeneric(data: *mut *mut std::ffi::c_void, size: usize) -> i32;
}

pub enum CudaMemCpy {
    DeviceToHost,
    HostToDevice,
}


fn cuda_malloc<T>(n: usize) -> *mut T {
    let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut();
    let status = unsafe {
        cudaMallocGeneric(&mut ptr as *mut *mut std::ffi::c_void, n* std::mem::size_of::<T>())
    };
    assert_eq!(status, 0, "cudaMalloc failed with status {}", status);
    ptr as *mut T
}


fn cuda_check(status: i32) {
    if status != 0 {
        panic!("CUDA Error: code {}", status)
    }
}


fn main() {

    let n = 1024;
    let mut a = vec![1.0f32; n];
    // let mut device_ptr: *mut f32 = std::ptr::null_mut();
    // let status = unsafe { cudaMallocF32( &mut device_ptr as *mut *mut f32, n as i32) };
    // assert!(!device_ptr.is_null());
    // println!("cudaMallocF32 succeeded! Device pointer: {:?}", device_ptr);
    let a_d: *mut f32 = cuda_malloc(n);


    // let status = unsafe { cudaFree(device_ptr as *mut std::ffi::c_void)};
    // println!("status: {:?}", status);

    // unsafe {
    //     cuda_check(status);
    // }
    // let n = 1024;
    // let mut b = vec![2.0f32; n];
    // let mut c = vec![0.0f32; n];

    // unsafe {
    //     cuda_add_kernel(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), n as i32);
    // }

    // unsafe {
    //     cudaMallocF32(a.as_mut_ptr(), n as i32);
    // }

    // println!("First few results: {:?}", &c[..10]);
}
