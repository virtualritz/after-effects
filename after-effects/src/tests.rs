//! Unit tests for types and functions that don't require After Effects host.

use super::*;

#[cfg(test)]
mod time_tests {
    use super::*;

    #[test]
    fn test_time_creation() {
        let t = Time { value: 30, scale: 1 };
        assert_eq!(t.value, 30);
        assert_eq!(t.scale, 1);
    }

    #[test]
    fn test_time_equality() {
        let t1 = Time { value: 1, scale: 2 };
        let t2 = Time { value: 1, scale: 2 };
        let t3 = Time { value: 2, scale: 4 };

        assert_eq!(t1, t2);
        assert_eq!(t1, t3); // 1/2 == 2/4 due to rational comparison
    }

    #[test]
    fn test_time_as_f64() {
        let t = Time { value: 45, scale: 30 };
        let seconds: f64 = t.into();
        assert!((seconds - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_time_addition_same_scale() {
        let t1 = Time { value: 10, scale: 30 };
        let t2 = Time { value: 20, scale: 30 };
        let result = t1 + t2;

        assert_eq!(result.value, 30);
        assert_eq!(result.scale, 30);
    }

    #[test]
    fn test_time_addition_different_scale() {
        let t1 = Time { value: 1, scale: 2 };  // 0.5
        let t2 = Time { value: 1, scale: 4 };  // 0.25
        let result = t1 + t2;

        // Result should be 3/4 = 0.75
        let seconds: f64 = result.into();
        assert!((seconds - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_time_zero() {
        let t = Time { value: 0, scale: 30 };
        let seconds: f64 = t.into();
        assert_eq!(seconds, 0.0);
    }
}

#[cfg(test)]
mod rect_tests {
    use super::*;

    #[test]
    fn test_rect_creation() {
        let r = Rect { left: 10, top: 20, right: 100, bottom: 200 };
        assert_eq!(r.left, 10);
        assert_eq!(r.top, 20);
        assert_eq!(r.right, 100);
        assert_eq!(r.bottom, 200);
    }

    #[test]
    fn test_rect_width_height() {
        let r = Rect { left: 10, top: 20, right: 100, bottom: 120 };
        assert_eq!(r.width(), 90);
        assert_eq!(r.height(), 100);
    }


    #[test]
    fn test_rect_contains_point() {
        let r = Rect { left: 0, top: 0, right: 100, bottom: 100 };

        assert!(r.contains(50, 50));
        assert!(r.contains(0, 0));
        assert!(r.contains(99, 99));
        assert!(!r.contains(100, 100)); // right/bottom are exclusive
        assert!(!r.contains(-1, 50));
        assert!(!r.contains(50, 101));
    }

    #[test]
    fn test_rect_union() {
        let mut r1 = Rect { left: 0, top: 0, right: 50, bottom: 50 };
        let r2 = Rect { left: 25, top: 25, right: 100, bottom: 100 };
        let union = r1.union(&r2);

        assert_eq!(union.left, 0);
        assert_eq!(union.top, 0);
        assert_eq!(union.right, 100);
        assert_eq!(union.bottom, 100);
    }


    #[test]
    fn test_rect_zero() {
        let r = Rect { left: 0, top: 0, right: 0, bottom: 0 };
        assert_eq!(r.width(), 0);
        assert_eq!(r.height(), 0);
    }

    #[test]
    fn test_rect_negative_dimensions() {
        // This shouldn't happen in normal use but test defensive behavior
        let r = Rect { left: 100, top: 100, right: 0, bottom: 0 };
        // Width and height should handle this gracefully
        let _w = r.width();
        let _h = r.height();
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point { h: 10, v: 20 };
        assert_eq!(p.h, 10);
        assert_eq!(p.v, 20);
    }

    #[test]
    fn test_point_equality() {
        let p1 = Point { h: 10, v: 20 };
        let p2 = Point { h: 10, v: 20 };
        let p3 = Point { h: 11, v: 20 };

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

}

#[cfg(test)]
mod error_tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_error_display() {
        let err = Error::OutOfMemory;
        let display = format!("{}", err);
        assert!(display.contains("memory"));
    }

    #[test]
    fn test_error_from_code() {
        let err = Error::from(ae_sys::A_Err_ALLOC as i32);
        assert_eq!(err, Error::OutOfMemory);
    }

    #[test]
    fn test_error_source() {
        let err = Error::InvalidCallback;
        assert!(err.source().is_none());
    }

    #[test]
    fn test_error_variants() {
        // Test that all error variants can be created
        let errors = vec![
            Error::None,
            Error::Generic,
            Error::Struct,
            Error::Parameter,
            Error::OutOfMemory,
            Error::WrongThread,
            Error::ConstProjectModification,
            Error::MissingSuite,
            Error::InternalStructDamaged,
            Error::InvalidIndex,
            Error::UnrecogizedParameterType,
            Error::InvalidCallback,
            Error::BadCallbackParameter,
        ];

        for err in errors {
            // Just ensure they can be constructed and displayed
            let _ = format!("{}", err);
        }
    }
}

#[cfg(test)]
mod fixed_point_tests {
    use super::*;

    #[test]
    fn test_fixed_from_f32() {
        let f = Fixed::from(1.5);
        // Fixed point with 16 bits of fractional precision
        // 1.5 * 65536 = 98304
        assert_eq!(f.as_fixed(), 98304);
    }

    #[test]
    fn test_fixed_to_f32() {
        let f = Fixed::from_fixed(65536); // 1.0 in fixed point
        let val: f32 = f.into();
        assert!((val - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_fixed_zero() {
        let f = Fixed::from(0.0);
        assert_eq!(f.as_fixed(), 0);
    }

    #[test]
    fn test_fixed_negative() {
        let f = Fixed::from(-1.5);
        assert!(f.as_fixed() < 0);
    }

    #[test]
    fn test_fixed_roundtrip() {
        let values = vec![0.0, 1.0, -1.0, 0.5, -0.5, 100.25, -100.25];

        for &val in &values {
            let f = Fixed::from(val);
            let result: f32 = f.into();
            assert!((result - val).abs() < 0.01, "Failed for value {}", val);
        }
    }
}

#[cfg(test)]
mod rational_scale_tests {
    use super::*;

    #[test]
    fn test_rational_scale_creation() {
        let rs = RationalScale { num: 16, den: 9 };
        assert_eq!(rs.num, 16);
        assert_eq!(rs.den, 9);
    }

}

#[cfg(test)]
mod pixel_tests {
    use super::*;

    #[test]
    fn test_pixel8_creation() {
        let p = Pixel8 {
            alpha: 255,
            red: 128,
            green: 64,
            blue: 32,
        };
        assert_eq!(p.alpha, 255);
        assert_eq!(p.red, 128);
        assert_eq!(p.green, 64);
        assert_eq!(p.blue, 32);
    }

    #[test]
    fn test_pixel16_creation() {
        let p = Pixel16 {
            alpha: 32768,
            red: 16384,
            green: 8192,
            blue: 4096,
        };
        assert_eq!(p.alpha, 32768);
    }

    #[test]
    fn test_pixel_f32_creation() {
        let p = PixelF32 {
            alpha: 1.0,
            red: 0.5,
            green: 0.25,
            blue: 0.125,
        };
        assert_eq!(p.alpha, 1.0);
        assert_eq!(p.red, 0.5);
    }

    #[test]
    fn test_pixel8_to_16() {
        let p8 = Pixel8 {
            alpha: 255,
            red: 128,
            green: 64,
            blue: 0,
        };
        let p16 = pixel8_to_16(p8);

        // 8-bit to 16-bit conversion: value * 257
        assert_eq!(p16.alpha, 65535); // 255 * 257
        assert_eq!(p16.red, 32896);   // 128 * 257
    }
}


#[cfg(test)]
mod try_reserve_error_tests {
    use super::*;

    #[test]
    fn test_try_reserve_error_conversion() {
        // This tests the TryReserveError -> Error conversion
        let mut vec = Vec::<u8>::new();
        // Try to reserve an impossibly large amount
        if let Err(e) = vec.try_reserve(usize::MAX) {
            let our_error: Error = e.into();
            assert_eq!(our_error, Error::OutOfMemory);
        }
    }
}
