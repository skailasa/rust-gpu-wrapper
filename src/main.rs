unsafe extern "C" {
    fn cuda_add_kernel(a: *mut f32, b: *mut f32, c: *mut f32, n: i32);
}

fn main() {
    let n = 1024;
    let mut a = vec![1.0f32; n];
    let mut b = vec![2.0f32; n];
    let mut c = vec![0.0f32; n];

    unsafe {
        cuda_add_kernel(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr(), n as i32);
    }

    println!("First few results: {:?}", &c[..10]);
}
