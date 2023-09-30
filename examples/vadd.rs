#![feature(riscv_target_feature)]

use rvv_std_demo::RVV;

#[inline(never)]
#[no_mangle]
#[target_feature(enable = "v")]
unsafe fn vadd(x: &[u8], y: &[u8], result: &mut [u8]) {
    assert!(x.len() == y.len() && x.len() == result.len());

    let mut remaining = x.len();

    let mut x = x as *const [u8] as *const u8;
    let mut y = y as *const [u8] as *const u8;
    let mut result = result as *mut [u8] as *mut u8;

    let mut i = 0;

    while remaining > 0 {
        <RVV<u8>>::new(remaining).go(|ctx| {
            let vl = ctx.vl;

            println!("[{i}]: VL = {vl}");
            i += 1;

            ctx.reg::<0>().load(x);
            ctx.reg::<1>().load(y);

            ctx.add::<2, 0, 1>();

            ctx.reg::<2>().store(result);

            remaining -= vl;

            unsafe {
                x = x.offset(vl as isize);
                y = y.offset(vl as isize);
                result = result.offset(vl as isize);
            }
        });
    }
}

fn main() {
    let mut result = [0u8; 3];

    unsafe { vadd(&[1, 2, 3], &[6, 3, 5], &mut result) };

    assert_eq!(result, [7, 5, 8]);

    let x: [u8; 435] = std::array::from_fn(|i| (i % 10) as u8);
    let y: [u8; 435] = std::array::from_fn(|i| (i % 23) as u8);
    let mut result = [0u8; 435];

    unsafe { vadd(&x, &y, &mut result) };

    assert_eq!(
        result,
        std::array::from_fn(|i| (i % 10) as u8 + (i % 23) as u8)
    );
}
