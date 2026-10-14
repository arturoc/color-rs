// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Color channel conversions and utility methods

use num_traits::{Float, zero, one};
use std::{u8, u16, u32};
use half::f16;

pub trait Channel: Copy + Sized + PartialOrd + PartialEq {
    type NearestFloat: Float + Channel;
    /// One for normalized floating-point channels, or the maximum integer value.
    const CHANNEL_MAX: Self;

    fn from<T:Channel>(chan: T) -> Self;
    fn to_channel<T:Channel>(self) -> T { Channel::from(self) }
    fn to_channel_u8(self)  -> u8;
    fn to_channel_u16(self) -> u16;
    fn to_channel_u32(self) -> u32;
    fn to_channel_f16(self) -> f16;
    fn to_channel_f32(self) -> f32;
    fn to_channel_f64(self) -> f64;
    fn to_nearest_precision_float(self) -> Self::NearestFloat;

    fn invert_channel(self) -> Self;

    fn clamp(self, lo: Self, hi: Self) -> Self {
        if self < lo {
            lo
        } else if self > hi {
            hi
        } else {
            self
        }
    }

    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        Channel::from(self.to_channel_f32() * rhs.to_channel_f32())
    }

    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        Channel::from(self.to_channel_f32() / rhs.to_channel_f32())
    }

    fn mix(self, rhs: Self, value: Self) -> Self;
    fn zero() -> Self;

    fn channel_min(self, rhs: Self) -> Self {
        if self < rhs { self } else { rhs }
    }

    fn channel_max(self, rhs: Self) -> Self {
        if self > rhs { self } else { rhs }
    }
}

impl Channel for u8 {
    type NearestFloat = f32;
    const CHANNEL_MAX: Self = u8::MAX;

    #[inline] fn from<T:Channel>(chan: T) -> u8 { chan.to_channel_u8() }
    #[inline] fn to_channel_u8(self)  -> u8  { self }
    #[inline] fn to_channel_u16(self) -> u16 { ((self as u16) << 8) | self as u16 }
    #[inline] fn to_channel_u32(self) -> u32 { self as u32 * 0x01010101 }
    #[inline] fn to_channel_f16(self) -> f16 { f16::from_f32(self.to_channel_f32()) }
    #[inline] fn to_channel_f32(self) -> f32 { self as f32 / u8::MAX as f32 }
    #[inline] fn to_channel_f64(self) -> f64 { self as f64 / u8::MAX as f64 }
    #[inline] fn to_nearest_precision_float(self) -> Self::NearestFloat{ self.to_channel_f32() }

    #[inline] fn invert_channel(self) -> u8 { !self }

    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        self + (rhs - self).normalized_mul(value)
    }
    fn zero() -> Self{
        0
    }
}

impl Channel for u16 {
    type NearestFloat = f32;
    const CHANNEL_MAX: Self = u16::MAX;

    #[inline] fn from<T:Channel>(chan: T) -> u16 { chan.to_channel_u16() }
    #[inline] fn to_channel_u8(self)  -> u8  { ((self as u32 + 128) / 257) as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { self }
    #[inline] fn to_channel_u32(self) -> u32 { ((self as u32) << 16) | self as u32 }
    #[inline] fn to_channel_f16(self) -> f16 { f16::from_f32(self.to_channel_f32()) }
    #[inline] fn to_channel_f32(self) -> f32 { self as f32 / u16::MAX as f32 }
    #[inline] fn to_channel_f64(self) -> f64 { self as f64 / u16::MAX as f64 }
    #[inline] fn to_nearest_precision_float(self) -> Self::NearestFloat{ self.to_channel_f32() }

    #[inline] fn invert_channel(self) -> u16 { !self }

    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        self + (rhs - self).normalized_mul(value)
    }
    fn zero() -> Self{
        0
    }
}

impl Channel for u32 {
    type NearestFloat = f32;
    const CHANNEL_MAX: Self = u32::MAX;

    #[inline] fn from<T:Channel>(chan: T) -> u32 { chan.to_channel_u32() }
    // Divide by u32::MAX / destination::MAX. These divisors are odd, so
    // exact halfway ties cannot occur.
    // Widen before adding the rounding offset to avoid overflow near u32::MAX.
    #[inline] fn to_channel_u8(self)  -> u8  { ((self as u64 + 8_421_504) / 16_843_009) as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { ((self as u64 + 32_768) / 65_537) as u16 }
    #[inline] fn to_channel_u32(self) -> u32 { self }
    #[inline] fn to_channel_f16(self) -> f16 { f16::from_f32(self.to_channel_f32()) }
    #[inline] fn to_channel_f32(self) -> f32 { self as f32 / u32::MAX as f32 }
    #[inline] fn to_channel_f64(self) -> f64 { self as f64 / u32::MAX as f64 }
    #[inline] fn to_nearest_precision_float(self) -> Self::NearestFloat{ self.to_channel_f32() }

    #[inline] fn invert_channel(self) -> u32 { !self }

    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        self + (rhs - self).normalized_mul(value)
    }
    fn zero() -> Self{
        0
    }
}

impl Channel for f16 {
    type NearestFloat = f32;
    const CHANNEL_MAX: Self = f16::ONE;

    #[inline] fn from<T:Channel>(chan: T) -> f16 { chan.to_channel_f16() }
    #[inline] fn to_channel_u8(self)  -> u8  { self.to_f32().to_channel_u8() }
    #[inline] fn to_channel_u16(self) -> u16 { self.to_f32().to_channel_u16() }
    #[inline] fn to_channel_u32(self) -> u32 { self.to_f32().to_channel_u32() }
    #[inline] fn to_channel_f16(self) -> f16 { self }
    #[inline] fn to_channel_f32(self) -> f32 { self.to_f32() }
    #[inline] fn to_channel_f64(self) -> f64 { self.to_f64() }
    #[inline] fn to_nearest_precision_float(self) -> Self::NearestFloat{ self.to_channel_f32() }

    #[inline] fn invert_channel(self) -> f16 { f16::from_f32(1.0 - self.to_f32()) }

    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        f16::from_f32(self.to_f32() * rhs.to_f32())
    }

    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        f16::from_f32(self.to_f32() / rhs.to_f32())
    }

    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        f16::from_f32(self.to_f32().mix(rhs.to_f32(), value.to_f32()))
    }
    fn zero() -> Self{
        f16::ZERO
    }
}

impl Channel for f32 {
    type NearestFloat = f32;
    const CHANNEL_MAX: Self = 1.;

    #[inline] fn from<T:Channel>(chan: T) -> f32 { chan.to_channel_f32() }
    #[inline] fn to_channel_u8(self)  -> u8  { (self * (u8::MAX as f32)).round_ties_even() as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { (self * (u16::MAX as f32)).round_ties_even() as u16 }
    #[inline] fn to_channel_u32(self) -> u32 { (self as f64 * (u32::MAX as f64)).round_ties_even() as u32 }
    #[inline] fn to_channel_f16(self) -> f16 { f16::from_f32(self) }
    #[inline] fn to_channel_f32(self) -> f32 { self }
    #[inline] fn to_channel_f64(self) -> f64 { self as f64 }
    #[inline] fn to_nearest_precision_float(self) -> Self::NearestFloat{ self }

    #[inline] fn invert_channel(self) -> f32 { 1.0 - self }

    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        self * rhs
    }

    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        self / rhs
    }

    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        self + (rhs - self).normalized_mul(value)
    }
    fn zero() -> Self{
        0.
    }
}

impl Channel for f64 {
    type NearestFloat = f64;
    const CHANNEL_MAX: Self = 1.;

    #[inline] fn from<T:Channel>(chan: T) -> f64 { chan.to_channel_f64() }
    #[inline] fn to_channel_u8(self)  -> u8  { (self * u8::MAX as f64).round_ties_even() as u8 }
    #[inline] fn to_channel_u16(self) -> u16 { (self * u16::MAX as f64).round_ties_even() as u16 }
    #[inline] fn to_channel_u32(self) -> u32 { (self * u32::MAX as f64).round_ties_even() as u32 }
    #[inline] fn to_channel_f16(self) -> f16 { f16::from_f64(self) }
    #[inline] fn to_channel_f32(self) -> f32 { self as f32 }
    #[inline] fn to_channel_f64(self) -> f64 { self }
    #[inline] fn to_nearest_precision_float(self) -> Self::NearestFloat{ self }

    #[inline] fn invert_channel(self) -> f64 { 1.0 - self }

    #[inline]
    fn normalized_mul(self, rhs: Self) -> Self {
        self * rhs
    }

    #[inline]
    fn normalized_div(self, rhs: Self) -> Self {
        self / rhs
    }

    #[inline]
    fn mix(self, rhs: Self, value: Self) -> Self {
        self + (rhs - self).normalized_mul(value)
    }
    fn zero() -> Self{
        0.
    }
}

pub trait FloatChannel: Float + Channel {
    #[inline]
    fn saturate(self) -> Self {
        Channel::clamp(self, zero(), one())
    }
}

impl FloatChannel for f32 {}
impl FloatChannel for f64 {}

/// Convert hue in degrees without applying normalized channel scaling.
#[inline]
pub(crate) fn cast_hue<T: num_traits::NumCast, U: Channel + num_traits::NumCast>(hue: T) -> U {
    // Floating-point channels have CHANNEL_MAX == 1 and retain fractional degrees.
    if U::CHANNEL_MAX.to_f64().unwrap() > 1.0 {
        let hue: f64 = num_traits::cast(hue).unwrap();
        // Wrap negative angles before the unsigned cast, and 360 back to zero
        // after rounding. The destination must still accommodate the hue in degrees.
        num_traits::cast(hue.rem_euclid(360.0).round_ties_even() % 360.0).unwrap()
    } else {
        num_traits::cast(hue).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Channel;

    #[test]
    fn test_to_channel_u8() {
        assert_eq!(0x00_u8.to_channel_u8(), 0x00_u8);
        assert_eq!(0x30_u8.to_channel_u8(), 0x30_u8);
        assert_eq!(0x66_u8.to_channel_u8(), 0x66_u8);
        assert_eq!(0xA0_u8.to_channel_u8(), 0xA0_u8);
        assert_eq!(0xFF_u8.to_channel_u8(), 0xFF_u8);

        assert_eq!(0x00_u8.to_channel_u16(), 0x0000_u16);
        assert_eq!(0x30_u8.to_channel_u16(), 0x3030_u16);
        assert_eq!(0x66_u8.to_channel_u16(), 0x6666_u16);
        assert_eq!(0xA0_u8.to_channel_u16(), 0xA0A0_u16);
        assert_eq!(0xFF_u8.to_channel_u16(), 0xFFFF_u16);

        assert_eq!(0x00_u8.to_channel_f32(), 0f32);
        assert_eq!(0xFF_u8.to_channel_f32(), 1f32);

        assert_eq!(0x00_u8.to_channel_f64(), 0f64);
        assert_eq!(0xFF_u8.to_channel_f64(), 1f64);
    }

    #[test]
    fn test_invert_channel_u8() {
        assert_eq!(0x00_u8.invert_channel(), 0xFF_u8);
        assert_eq!(0x66_u8.invert_channel(), 0x99_u8);
        assert_eq!(0xFF_u8.invert_channel(), 0x00_u8);
    }

    #[test]
    fn test_to_channel_u16() {
        assert_eq!(0x0000_u16.to_channel_u8(), 0x00_u8);
        assert_eq!(0x3300_u16.to_channel_u8(), 0x33_u8);
        assert_eq!(0x6666_u16.to_channel_u8(), 0x66_u8);
        assert_eq!(0xAA00_u16.to_channel_u8(), 0xA9_u8);
        assert_eq!(0xFFFF_u16.to_channel_u8(), 0xFF_u8);

        assert_eq!(0x0000_u16.to_channel_u16(), 0x0000_u16);
        assert_eq!(0x3300_u16.to_channel_u16(), 0x3300_u16);
        assert_eq!(0x6666_u16.to_channel_u16(), 0x6666_u16);
        assert_eq!(0xAA00_u16.to_channel_u16(), 0xAA00_u16);
        assert_eq!(0xFFFF_u16.to_channel_u16(), 0xFFFF_u16);

        assert_eq!(0x0000_u16.to_channel_f32(), 0f32);
        assert_eq!(0xFFFF_u16.to_channel_f32(), 1f32);

        assert_eq!(0x0000_u16.to_channel_f64(), 0f64);
        assert_eq!(0xFFFF_u16.to_channel_f64(), 1f64);
    }

    #[test]
    fn test_invert_channel_u16() {
        assert_eq!(0x0000_u16.invert_channel(), 0xFFFF_u16);
        assert_eq!(0x6666_u16.invert_channel(), 0x9999_u16);
        assert_eq!(0xFFFF_u16.invert_channel(), 0x0000_u16);
    }

    #[test]
    fn test_integer_channel_scaling() {
        assert_eq!(0x12u8.to_channel_u32(), 0x12121212);
        assert_eq!(0x1234u16.to_channel_u32(), 0x12341234);
        assert_eq!(u8::MAX.to_channel_u32(), u32::MAX);
        assert_eq!(u16::MAX.to_channel_u32(), u32::MAX);

        for value in 0..=u16::MAX {
            assert_eq!(value.to_channel_u32().to_channel_u16(), value);
            assert_eq!(value.to_channel_u8(), value.to_channel_f64().to_channel_u8());
        }
        for value in 0..=u8::MAX {
            assert_eq!(value.to_channel_u32().to_channel_u8(), value);
        }
    }

    #[test]
    fn test_integer_channel_rounding_boundaries() {
        assert_eq!(128u16.to_channel_u8(), 0);
        assert_eq!(129u16.to_channel_u8(), 1);
        assert_eq!(65406u16.to_channel_u8(), 254);
        assert_eq!(65407u16.to_channel_u8(), 255);
        assert_eq!(8_421_504u32.to_channel_u8(), 0);
        assert_eq!(8_421_505u32.to_channel_u8(), 1);
        assert_eq!(32_768u32.to_channel_u16(), 0);
        assert_eq!(32_769u32.to_channel_u16(), 1);
        assert_eq!(0x80000000u32.to_channel_u8(), 128);
        assert_eq!(0x80000000u32.to_channel_u16(), 32768);
        for value in [0, 1, 0x12345678, 0x7fffffff, u32::MAX - 1, u32::MAX] {
            assert_eq!(value.to_channel_u8(), value.to_channel_f64().to_channel_u8());
            assert_eq!(value.to_channel_u16(), value.to_channel_f64().to_channel_u16());
        }
    }

    #[test]
    fn test_float_channel_ties_and_u32_precision() {
        for (scaled, expected) in [(2.5f64, 2u32), (3.5, 4)] {
            assert_eq!((scaled / 255.0).to_channel_u8(), expected as u8);
            assert_eq!((scaled as f32 / 255.0).to_channel_u8(), expected as u8);
            assert_eq!((scaled / 65535.0).to_channel_u16(), expected as u16);
            assert_eq!((scaled as f32 / 65535.0).to_channel_u16(), expected as u16);
            assert_eq!((scaled / u32::MAX as f64).to_channel_u32(), expected);
        }
        assert_eq!(0.75f32.to_channel_u32(), 0xbfffffff);
        assert_eq!(0.75f64.to_channel_u32(), 0xbfffffff);
        assert_eq!(half::f16::from_f32(0.75).to_channel_u32(), 0xbfffffff);
        assert_eq!(half::f16::from_f32(0.5).to_channel_u8(), 128);
        assert_eq!(half::f16::from_f32(0.5).to_channel_u16(), 32768);
    }

    #[test]
    fn test_float_channel_saturation() {
        fn check<T: Channel>(value: T, expected: (u8, u16, u32)) {
            assert_eq!((value.to_channel_u8(), value.to_channel_u16(), value.to_channel_u32()), expected);
        }
        for (value, expected) in [
            (f64::NAN, (0, 0, 0)),
            (f64::NEG_INFINITY, (0, 0, 0)),
            (-1.0, (0, 0, 0)),
            (0.0, (0, 0, 0)),
            (1.0, (u8::MAX, u16::MAX, u32::MAX)),
            (2.0, (u8::MAX, u16::MAX, u32::MAX)),
            (f64::INFINITY, (u8::MAX, u16::MAX, u32::MAX)),
        ] {
            check(value, expected);
            check(value as f32, expected);
            check(half::f16::from_f64(value), expected);
        }
    }

    #[test]
    fn test_to_channel_f32() {
        assert_eq!(0.00f32.to_channel_u8(), 0x00);
        assert_eq!(0.25f32.to_channel_u8(), 0x40);
        assert_eq!(0.50f32.to_channel_u8(), 0x80);
        assert_eq!(0.75f32.to_channel_u8(), 0xBF);
        assert_eq!(1.00f32.to_channel_u8(), 0xFF);

        assert_eq!(0.00f32.to_channel_u16(), 0x0000);
        assert_eq!(0.25f32.to_channel_u16(), 0x4000);
        assert_eq!(0.50f32.to_channel_u16(), 0x8000);
        assert_eq!(0.75f32.to_channel_u16(), 0xBFFF);
        assert_eq!(1.00f32.to_channel_u16(), 0xFFFF);

        assert_eq!(0.00f32.to_channel_f32(), 0.00f32);
        assert_eq!(1.00f32.to_channel_f32(), 1.00f32);

        assert_eq!(0.00f32.to_channel_f64(), 0.00f64);
        assert_eq!(1.00f32.to_channel_f64(), 1.00f64);
    }

    #[test]
    fn test_invert_channel_f32() {
        assert_eq!(0.00f32.invert_channel(), 1.00f32);
        assert_eq!(0.50f32.invert_channel(), 0.50f32);
        assert_eq!(1.00f32.invert_channel(), 0.00f32);
    }

    #[test]
    fn test_to_channel_f64() {
        assert_eq!(0.00f64.to_channel_u8(), 0x00);
        assert_eq!(0.25f64.to_channel_u8(), 0x40);
        assert_eq!(0.50f64.to_channel_u8(), 0x80);
        assert_eq!(0.75f64.to_channel_u8(), 0xBF);
        assert_eq!(1.00f64.to_channel_u8(), 0xFF);

        assert_eq!(0.00f64.to_channel_u16(), 0x0000);
        assert_eq!(0.25f64.to_channel_u16(), 0x4000);
        assert_eq!(0.50f64.to_channel_u16(), 0x8000);
        assert_eq!(0.75f64.to_channel_u16(), 0xBFFF);
        assert_eq!(1.00f64.to_channel_u16(), 0xFFFF);

        assert_eq!(0.00f64.to_channel_f32(), 0.00f32);
        assert_eq!(1.00f64.to_channel_f32(), 1.00f32);

        assert_eq!(0.00f64.to_channel_f64(), 0.00f64);
        assert_eq!(1.00f64.to_channel_f64(), 1.00f64);
    }

    #[test]
    fn test_invert_channel_f64() {
        assert_eq!(0.00f64.invert_channel(), 1.00f64);
        assert_eq!(0.50f64.invert_channel(), 0.50f64);
        assert_eq!(1.00f64.invert_channel(), 0.00f64);
    }
}
