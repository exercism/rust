use hexlit::hex;
#[cfg(feature = "io")]
use std::io::{Read, Write};
use xorcism::Xorcism;

#[test]
fn munge_in_place_identity() {
    let mut xs = Xorcism::new(&[0]);
    let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();
    let mut output = input.to_owned();
    xs.munge_in_place(&mut output);

    assert_eq!(&input, &output);
}

#[test]
#[ignore]
fn munge_in_place_roundtrip() {
    let mut xs1 = Xorcism::new(&[1, 2, 3, 4, 5]);
    let mut xs2 = Xorcism::new(&[1, 2, 3, 4, 5]);
    let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();
    let mut cipher = input.to_owned();
    xs1.munge_in_place(&mut cipher);
    assert_ne!(&input, &cipher);
    let mut output = cipher;
    xs2.munge_in_place(&mut output);
    assert_eq!(&input, &output);
}

#[test]
#[ignore]
fn munge_in_place_stateful() {
    let mut xs = Xorcism::new(&[1, 2, 3, 4, 5]);
    let input = "This is super-secret, cutting edge encryption, folks.".as_bytes();

    let mut cipher1 = input.to_owned();
    let mut cipher2 = input.to_owned();
    xs.munge_in_place(&mut cipher1);
    xs.munge_in_place(&mut cipher2);

    assert_ne!(&input, &cipher1);
    assert_ne!(&input, &cipher2);
    assert_ne!(&cipher1, &cipher2);
}

#[test]
#[ignore]
fn munge_identity() {
    let mut xs = Xorcism::new(&[0]);
    let data = "This is super-secret, cutting edge encryption, folks.";

    assert_eq!(
        xs.munge(data.as_bytes()).collect::<Vec<_>>(),
        data.as_bytes()
    );
}

#[test]
#[ignore]
fn statefulness() {
    // we expect Xorcism to be stateful: at the end of a munging run, the key has rotated.
    // this means that until the key has completely rotated around, equal inputs will produce
    // unequal outputs.
    let key = &[0, 1, 2, 3, 4, 5, 6, 7];
    let input = &[0b1010_1010, 0b0101_0101];

    let mut xs = Xorcism::new(&key);
    let out1: Vec<_> = xs.munge(input).collect();
    let out2: Vec<_> = xs.munge(input).collect();
    let out3: Vec<_> = xs.munge(input).collect();
    let out4: Vec<_> = xs.munge(input).collect();
    let out5: Vec<_> = xs.munge(input).collect();

    assert_ne!(out1, out2);
    assert_ne!(out2, out3);
    assert_ne!(out3, out4);
    assert_ne!(out4, out5);
    assert_eq!(out1, out5);
}

macro_rules! test_cases {
    ($($name:ident, $key:literal, $input:literal, $expect:literal);+) => {
        $(mod $name {
            use super::*;

            const KEY: &str = $key;
            const INPUT: &str = $input;
            const EXPECT: &[u8] = &hex!($expect);

            /// tests where the key is expressed as `&str`, and input is expressed as `&[u8]`
            mod str_slice {
                use super::*;

                #[test]
                #[ignore]
                fn munge_in_place() {
                    // we transform the input into a `Vec<u8>` despite its presence in this
                    // module because of the more restricted syntax that this function accepts
                    let mut input = INPUT.as_bytes().to_vec();
                    let original = input.clone();

                    // in-place munging is stateful on Xorcism, so clone it
                    // to ensure the keys positions stay synchronized
                    let mut xorcism1 = Xorcism::new(KEY);
                    let mut xorcism2 = xorcism1.clone();

                    xorcism1.munge_in_place(&mut input);
                    assert_eq!(input.len(), original.len());
                    assert_ne!(input, original);
                    assert_eq!(input, EXPECT);
                    xorcism2.munge_in_place(&mut input);
                    assert_eq!(input, original);
                }

                #[test]
                #[ignore]
                fn munges() {
                    let mut xorcism = Xorcism::new(KEY);
                    let result: Vec<u8> = xorcism.munge(INPUT.as_bytes()).collect();
                    assert_eq!(INPUT.len(), result.len());
                    assert_ne!(INPUT.as_bytes(), result);
                    assert_eq!(result, EXPECT);
                }

                #[test]
                #[ignore]
                fn round_trip() {
                    let mut xorcism1 = Xorcism::new(KEY);
                    let mut xorcism2 = xorcism1.clone();
                    let munge_iter = xorcism1.munge(INPUT.as_bytes());
                    let result: Vec<u8> = xorcism2.munge(munge_iter).collect();
                    assert_eq!(INPUT.as_bytes(), result);
                }
            }

            /// tests where the key and input are both expressed as `&[u8]`
            mod slice_slice {
                use super::*;

                #[test]
                #[ignore]
                fn munge_in_place() {
                    let key = KEY.as_bytes();

                    // we transform the input into a `Vec<u8>` despite its presence in this
                    // module because of the more restricted syntax that this function accepts
                    let mut input = INPUT.as_bytes().to_vec();
                    let original = input.clone();

                    // in-place munging is stateful on Xorcism, so clone it
                    // to ensure the keys positions stay synchronized
                    let mut xorcism1 = Xorcism::new(key);
                    let mut xorcism2 = xorcism1.clone();

                    xorcism1.munge_in_place(&mut input);
                    assert_eq!(input.len(), original.len());
                    assert_ne!(input, original);
                    assert_eq!(input, EXPECT);
                    xorcism2.munge_in_place(&mut input);
                    assert_eq!(input, original);
                }

                #[test]
                #[ignore]
                fn munges() {
                    let key = KEY.as_bytes();
                    let input = INPUT.as_bytes();

                    let mut xorcism = Xorcism::new(key);
                    let result: Vec<u8> = xorcism.munge(input).collect();
                    assert_eq!(input.len(), result.len());
                    assert_ne!(input, result);
                    assert_eq!(result, EXPECT);
                }

                #[test]
                #[ignore]
                fn round_trip() {
                    let key = KEY.as_bytes();
                    let input = INPUT.as_bytes();

                    let mut xorcism1 = Xorcism::new(key);
                    let mut xorcism2 = xorcism1.clone();
                    let munge_iter = xorcism1.munge(input);
                    let result: Vec<u8> = xorcism2.munge(munge_iter).collect();
                    assert_eq!(input, result);
                }
            }

            /// tests where the key is expressed as `&str` and input is expressed as `Vec<u8>`
            mod vec_vec {
                use super::*;

                #[test]
                #[ignore]
                fn munge_in_place() {
                    let mut input = INPUT.as_bytes().to_vec();
                    let original = input.clone();

                    // in-place munging is stateful on Xorcism, so clone it
                    // to ensure the keys positions stay synchronized
                    let mut xorcism1 = Xorcism::new(KEY);
                    let mut xorcism2 = xorcism1.clone();

                    xorcism1.munge_in_place(&mut input);
                    assert_eq!(input.len(), original.len());
                    assert_ne!(input, original);
                    assert_eq!(input, EXPECT);
                    xorcism2.munge_in_place(&mut input);
                    assert_eq!(input, original);
                }

                #[test]
                #[ignore]
                fn munges() {
                    let owned_input = INPUT.as_bytes().to_vec();

                    let mut xorcism = Xorcism::new(KEY);
                    let result: Vec<u8> = xorcism.munge(owned_input).collect();
                    assert_eq!(INPUT.len(), result.len());
                    assert_ne!(INPUT.as_bytes(), result);
                    assert_eq!(result, EXPECT);
                }

                #[test]
                #[ignore]
                fn round_trip() {
                    let owned_input = INPUT.as_bytes().to_vec();

                    let mut xorcism1 = Xorcism::new(KEY);
                    let mut xorcism2 = xorcism1.clone();
                    let munge_iter = xorcism1.munge(owned_input);
                    let result: Vec<u8> = xorcism2.munge(munge_iter).collect();
                    assert_eq!(INPUT.as_bytes(), result);
                }
            }

            #[cfg(feature = "io")]
            mod io {
                use super::*;

                #[test]
                #[ignore]
                fn reader_munges() {
                    let mut reader = Xorcism::new(KEY).reader(INPUT.as_bytes());
                    let mut buf = Vec::with_capacity(INPUT.len());
                    let bytes_read = reader.read_to_end(&mut buf).unwrap();
                    assert_eq!(bytes_read, INPUT.len());
                    assert_eq!(buf, EXPECT);
                }

                #[test]
                #[ignore]
                fn reader_roundtrip() {
                    let xs = Xorcism::new(KEY);
                    let reader1 = xs.clone().reader(INPUT.as_bytes());
                    let mut reader2 = xs.clone().reader(reader1);
                    let mut buf = Vec::with_capacity(INPUT.len());
                    let bytes_read = reader2.read_to_end(&mut buf).unwrap();
                    assert_eq!(bytes_read, INPUT.len());
                    assert_eq!(buf, INPUT.as_bytes());
                }

                #[test]
                #[ignore]
                fn writer_munges() {
                    let mut writer_dest = Vec::new();
                    {
                        let mut writer = Xorcism::new(KEY).writer(&mut writer_dest);
                        assert!(writer.write_all(INPUT.as_bytes()).is_ok());
                    }
                    assert_eq!(writer_dest, EXPECT);
                }

                #[test]
                #[ignore]
                fn writer_roundtrip() {
                    let mut writer_dest = Vec::new();
                    let xs = Xorcism::new(KEY);
                    {
                        let writer1 = xs.clone().writer(&mut writer_dest);
                        let mut writer2 = xs.writer(writer1);
                        assert!(writer2.write_all(INPUT.as_bytes()).is_ok());
                    }
                    assert_eq!(writer_dest, INPUT.as_bytes());
                }
            }
        })+
    };
}

test_cases!(
    key_shorter_than_data, "abcde", "123455", "5050 5050 5054"
    ;
    key_len_equal_to_data,
        "The quick brown fox jumped over the lazy dog.",
        "Wait, oops, this is not the pangram exercise!",
        "0309 0c54 5d55 060c 1b53 4e52 1b1f 0753 4606 0b00 041a 1950 110c 454f 0604 1c47 0609 0800 0919 1f0b 430d 1c02 0f"
    ;
    key_longer_than_data,
        "A properly cryptographically random key longer than the data can actually be fairly secure.",
        "Text is not cryptographically random.",
        "1545 0806 4f19 1652 0216 5443 110b 0904 1b08 1513 1118 010a 020d 0015 5952 130f 0a0b 024d 45"
    ;
    shakespearean,
        "Forsooth, let us never break our trust!",
        "The sacred brothership in which we share shall never from our hearts be lost.",
        "1207 1753 1c0e 171a 4944 4c07 064f 011b 451c 161e 0c02 000b 1c45 1603 490c 1d52 5711 5206 1b15 5323 4f01 1b0e 0318 4842 451a 0006 0013 014f 0345 1910 0000 0a17 0413 1f53 4f17 1700 181d 0607 5a"
    ;
    comics,
        "Who knows what evil lurks in the hearts of men?",
        "Spiderman! It's spiderman! Not a bird, or a plane, or a fireman! Just spiderman!",
        "0418 0644 0e1c 0216 1d01 5721 1553 5345 0519 0544 0907 1f0a 1d01 4920 4f00 4804 000a 0c13 1658 534f 1d46 414d 1502 5e39 0d43 0004 1c4f 1653 461e 1a04 1941 0b57 4926 551f 0152 1803 490d 0b52 1909 0b01"
    ;
    mad_science,
        "TRANSMUTATION_NOTES_1",
        "If wishes were horses, beggars would ride.",
        "1d34 6139 3a3e 3d31 3274 3e2a 3c3a 6e27 3b37 203a 4278 7223 2b34 2a34 2632 743e 203b 332a 6f26 2c37 3a1f"
    ;
    metaphor,
        "Contextualism",
        "The globe is text, its people prose; all the world's a page.",
        "1707 0b54 0214 1b17 044c 0000 4d37 0a16 0049 581d 0112 4c19 1602 3303 0b54 150a 1b06 0457 4912 012f 4f1a 1c00 5803 1a13 000d 541e 630e 4e04 041f 115b"
    ;
    emoji,
        "🔑🗝️… 🎹?",
        "⌨️! 🔒+💻+🧠=🔓",
        "1213 3c7e 4810 b6bd 1f27 1b70 ab56 bf62 24a5 49a0 573f a961 6f0b 04"
);
// note the emoji case above. Most exercism tests don't test emoji, because we like to use strings
// as input, but in this case, they're fine: they're arbitrary binary data that _just happen_ to
// represent emoji when construed as a string, in this case.
