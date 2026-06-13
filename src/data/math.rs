/// Mathematical utility functions ported from Maths.cs
/// These are used by PlanetAlgorithm implementations.

/// Clamp a value between min and max (inclusive).
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Clamp a value between 0.0 and 1.0.
#[inline]
pub fn clamp01(value: f64) -> f64 {
    clamp(value, 0.0, 1.0)
}

/// C# Levelize: smoothstep-based levelize
/// f = f / level - offset;
/// num1 = floor(f);
/// num2 = f - num1;
/// num3 = (3.0 - num2 - num2) * num2 * num2; (smoothstep)
/// f = num1 + num3;
/// f = (f + offset) * level;
#[inline]
pub fn levelize(f: f64, level: f64, offset: f64) -> f64 {
    let f = f / level - offset;
    let num1 = f.floor();
    let num2 = f - num1;
    let num3 = (3.0 - num2 - num2) * num2 * num2;
    let f = num1 + num3;
    (f + offset) * level
}

/// C# Levelize2: smoothstep applied twice (smoother steps)
#[inline]
pub fn levelize2(f: f64, level: f64, offset: f64) -> f64 {
    let f = f / level - offset;
    let num1 = f.floor();
    let num2 = f - num1;
    let num3 = (3.0 - num2 - num2) * num2 * num2;
    let num4 = (3.0 - num3 - num3) * num3 * num3;
    let f = num1 + num4;
    (f + offset) * level
}

/// C# Levelize3: smoothstep applied three times (even smoother steps)
#[inline]
pub fn levelize3(f: f64, level: f64, offset: f64) -> f64 {
    let f = f / level - offset;
    let num1 = f.floor();
    let num2 = f - num1;
    let num3 = (3.0 - num2 - num2) * num2 * num2;
    let num4 = (3.0 - num3 - num3) * num3 * num3;
    let num5 = (3.0 - num4 - num4) * num4 * num4;
    let f = num1 + num5;
    (f + offset) * level
}

/// C# Levelize4: smoothstep applied four times (even smoother steps)
/// Uses floor() for correct negative-value handling.
#[inline]
pub fn levelize4(f: f64, level: f64, offset: f64) -> f64 {
    let f = f / level - offset;
    let num1 = f.floor();
    let num2 = f - num1;
    let num3 = (3.0 - num2 - num2) * num2 * num2;
    let num4 = (3.0 - num3 - num3) * num3 * num3;
    let num5 = (3.0 - num4 - num4) * num4 * num4;
    let num6 = (3.0 - num5 - num5) * num5 * num5;
    let f = num1 + num6;
    (f + offset) * level
}
