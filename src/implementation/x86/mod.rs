#[allow(dead_code)]
pub(crate) mod avx2;

#[allow(dead_code)]
pub(crate) mod sse42;

// validate_utf8_basic() std: implementation auto-selection

#[cfg(all(feature = "std", not(target_feature = "avx2")))]
#[inline]
pub(crate) unsafe fn validate_utf8_basic(
    input: &[u8],
) -> core::result::Result<(), crate::basic::Utf8Error> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), crate::basic::Utf8Error> {
        let fun = get_fastest_available_implementation_basic();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, super::ValidateUtf8Fn>(fun)(input)
}

#[cfg(all(feature = "std", not(target_feature = "avx2")))]
#[inline]
fn get_fastest_available_implementation_basic() -> super::ValidateUtf8Fn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_basic
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_basic
    } else {
        super::validate_utf8_basic_fallback
    }
}

// validate_utf8_basic() no-std: implementation selection by config

#[cfg(target_feature = "avx2")]
pub(crate) use avx2::validate_utf8_basic;

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
pub(crate) use sse42::validate_utf8_basic;

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
pub(crate) use super::validate_utf8_basic_fallback as validate_utf8_basic;

// validate_utf8_compat() std: implementation auto-selection

#[cfg(all(feature = "std", not(target_feature = "avx2")))]
#[cfg(feature = "std")]
#[inline]
pub(crate) unsafe fn validate_utf8_compat(
    input: &[u8],
) -> core::result::Result<(), crate::compat::Utf8Error> {
    use core::mem;
    use std::sync::atomic::{AtomicPtr, Ordering};

    type FnRaw = *mut ();

    static FN: AtomicPtr<()> = AtomicPtr::new(get_fastest as FnRaw);

    unsafe fn get_fastest(input: &[u8]) -> core::result::Result<(), crate::compat::Utf8Error> {
        let fun = get_fastest_available_implementation_compat();
        FN.store(fun as FnRaw, Ordering::Relaxed);
        (fun)(input)
    }

    let fun = FN.load(Ordering::Relaxed);
    mem::transmute::<FnRaw, super::ValidateUtf8CompatFn>(fun)(input)
}

#[cfg(all(feature = "std", not(target_feature = "avx2")))]
#[inline]
fn get_fastest_available_implementation_compat() -> super::ValidateUtf8CompatFn {
    if std::is_x86_feature_detected!("avx2") {
        avx2::validate_utf8_compat
    } else if std::is_x86_feature_detected!("sse4.2") {
        sse42::validate_utf8_compat
    } else {
        super::validate_utf8_compat_fallback
    }
}

// validate_utf8_basic() no-std: implementation selection by config

#[cfg(target_feature = "avx2")]
pub(crate) use avx2::validate_utf8_compat;

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    target_feature = "sse4.2"
))]
pub(crate) use sse42::validate_utf8_compat;

#[cfg(all(
    not(feature = "std"),
    not(target_feature = "avx2"),
    not(target_feature = "sse4.2")
))]
pub(crate) use super::validate_utf8_compat_fallback as validate_utf8_compat;
