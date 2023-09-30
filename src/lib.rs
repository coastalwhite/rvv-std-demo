#![feature(asm_const)]

use core::arch::asm;
use core::marker::PhantomData;

pub struct RVV<E> {
    avl: usize,
    _phantom: PhantomData<E>,
}
pub struct RVVContext<'a, E> {
    pub vl: usize,
    _rvv: &'a RVV<E>,
    _phantom: PhantomData<E>,
}

macro_rules! implement_rvv {
    ($($e:ty, $elem:literal),+ $(,)?) => {
        $(
        impl RVV<$e> {
            #[inline(always)]
            pub fn new(avl: usize) -> Self {
                Self { avl, _phantom: PhantomData::default() }
            }

            #[inline(always)]
            pub fn go(self, f: impl FnOnce(RVVContext<$e>)) {
                let mut vl: usize;

                unsafe {
                    core::arch::asm!(
                        concat!("vsetvli {vl},{avl},", $elem, ",m1,ta,ma"),
                        vl = lateout(reg) vl,
                        avl = in(reg) self.avl,
                        options(nomem, nostack),
                    )
                }

                f(RVVContext {
                    vl,
                    _rvv: &self,
                    _phantom: PhantomData::default(),
                })
            }
        }
        )+
    }
}

implement_rvv! {
    u8, "e8",
    u16, "e16",
    u32, "e32",
    u64, "e64",
}

impl<'a, E> RVVContext<'a, E> {
    #[inline(always)]
    pub fn vse8_v<const VD: usize>(&self, rs: *mut u8) {
        assert!(VD < 32);

        unsafe { asm!("vse8.v v{vd},({rs})", vd = const VD, rs = in(reg) rs) }
    }

    #[inline(always)]
    pub fn vle8_v<const VD: usize>(&self, rs: *const u8) {
        assert!(VD < 32);

        unsafe { asm!("vle8.v v{vd},({rs})", vd = const VD, rs = in(reg) rs) }
    }

    #[inline(always)]
    pub fn vadd_vv<const VD: usize, const VS1: usize, const VS2: usize>(&self) {
        assert!(VD < 32);
        assert!(VS1 < 32);
        assert!(VS2 < 32);

        unsafe {
            asm!("vadd.vv v{vd},v{vs1},v{vs2}", vd = const VD, vs1 = const VS1, vs2 = const VS2, options(nomem, nostack))
        }
    }
}
