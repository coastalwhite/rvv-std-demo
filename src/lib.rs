use core::arch::asm;
use core::marker::PhantomData;

pub struct RVV<E> {
    avl: usize,
    _phantom: PhantomData<E>,
}
pub struct RVVContext<E> {
    pub vl: usize,
    _phantom: PhantomData<E>,
}
#[derive(Clone, Copy)]
pub struct VRegister<const N: usize, E>(PhantomData<([(); N], E)>);

macro_rules! impl_regs {
    ($e:ty, $elem:literal, [$($n:literal),+]) => {
        $(
        impl VRegister<$n, $e> {
            #[inline(always)]
            pub fn load(self, elems: *const $e) {
                unsafe {
                    core::arch::asm!(
                        concat!("vl", $elem, ".v v", stringify!($n), ",({elems})"),
                        elems = in(reg) elems,
                    );
                }
            }

            #[inline(always)]
            pub fn store(self, elems: *mut $e) {
                unsafe {
                    core::arch::asm!(
                        concat!("vs", $elem, ".v v", stringify!($n), ",({elems})"),
                        elems = in(reg) elems,
                    );
                }
            }
        }
        )+
    }
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
                    _phantom: PhantomData::default(),
                })
            }
        }

        impl_regs!(
            $e, $elem,
            [
                0, 1, 2, 3, 4, 5, 6, 7,
                8, 9, 10, 11, 12, 13, 14, 15,
                16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31
            ]
        );
        )+
    }
}

implement_rvv! {
    u8, "e8",
    u16, "e16",
    u32, "e32",
    u64, "e64",
}

impl<E> RVVContext<E> {
    #[inline(always)]
    pub fn reg<const N: usize>(&self) -> VRegister<N, E> {
        assert!(N < 32);
        VRegister(PhantomData::default())
    }

    #[inline(always)]
    pub fn add<const VD: usize, const VS1: usize, const VS2: usize>(&self) {
        assert!(VD < 32);
        assert!(VS1 < 32);
        assert!(VS2 < 32);

        unsafe { asm!("vadd.vv v2,v0,v1", options(nomem, nostack)) }
    }
}
