use std::fmt;
use std::str::FromStr;

/// The number of decimal places (decimals) for WAVES is 8.
const DECIMALS: usize = 8;

/// The [`Amount`] type can be used to express Waves amounts that supports conversion to various denominations.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount(u64);

impl Amount {
    /// The zero amount.
    pub const ZERO: Amount = Amount(0);
    /// Exactly one WAVELET.
    pub const ONE_WAVELET: Amount = Amount(1);
    /// Exactly one WAVES.
    pub const ONE_WAVES: Amount = Amount(100_000_000);

    /// Create an [`Amount`] with WAVELET precision and the given number of WAVELET.
    pub fn from_wavelet(wavelet: u64) -> Amount {
        Amount(wavelet)
    }

    /// The maximum value of an [`Amount`].
    pub fn max_value() -> Amount {
        Amount(u64::max_value())
    }

    /// The minimum value of an [`Amount`].
    pub fn min_value() -> Amount {
        Amount(u64::min_value())
    }

    /// Get the number of WAVELET in this [`Amount`].
    pub fn as_wavelet(self) -> u64 {
        self.0
    }

    /// Express this [`Amount`] as a floating-point value in WAVES.
    pub fn as_waves(self) -> f64 {
        let real = format!("{:0width$}", self.0, width = DECIMALS);

        if real.len() == DECIMALS {
            let result = format!("0.{}", &real[real.len() - DECIMALS..]);
            f64::from_str(&result).unwrap()
        } else {
            let result = format!(
                "{}.{}",
                &real[0..(real.len() - DECIMALS)],
                &real[real.len() - DECIMALS..]
            );
            f64::from_str(&result).unwrap()
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Amount::ZERO
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_waves())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let wavelet = 1_000;
        let balance = Amount::from_wavelet(wavelet);
        assert_eq!(balance.as_waves(), 0.00001);

        let wavelet = 1_000_000;
        let balance = Amount::from_wavelet(wavelet);
        assert_eq!(balance.as_waves(), 0.01);

        let wavelet = 1_000_000_000;
        let balance = Amount::from_wavelet(wavelet);
        assert_eq!(balance.as_waves(), 10.0);
    }

    #[test]
    fn test_one_waves() {
        let one_waves = Amount::ONE_WAVES;
        assert_eq!(one_waves.as_waves(), 1.0);
    }
}
