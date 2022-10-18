use bip39::{Language, Mnemonic, MnemonicType};

/// Seed phrase generation function
///
/// The private key can be generated from some random seed phrase using hashing functions. The public key is obtained from the private key using an elliptic curve multiplication. Account address is obtained from the public key. All these transformations are unidirectional. The opposite direction is almost impossible in terms of the required computations.
///
/// The secret phrase (a.k.a. seed phrase, backup phrase) can be any combination of symbols, words, or bytes. Waves wallet apps typically use a random set of 15 English words out of 2048 words available. Using such a phrase is secure: the probability of generating two identical seed phrases is 1/204815, so brute-force will take millions of years on an average CPU. The point of using a secret phrase (rather than a private key) is to simplify user experience: the secret phrase is much easier to write down or remember.
///
/// Example of a secret phrase:
/// ```plain_text
/// body key praise enter toss road cup result shrimp bus blame typical sphere pottery claim
/// ```
///
/// # Usage
/// ```
/// use wavesplatform::seed::*;
/// let phrase = generate_phrase();
/// ```
pub fn generate_phrase() -> String {
    let mnemonic = Mnemonic::new(MnemonicType::Words18, Language::English);
    let phrase: &str = mnemonic.phrase();
    phrase.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phrase() {
        let phrase = generate_phrase();

        assert_eq!(phrase.split_ascii_whitespace().count(), 18);
    }
}
