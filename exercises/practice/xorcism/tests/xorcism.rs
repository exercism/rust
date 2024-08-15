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

mod key_shorter_than_data {

    use super::*;

    const KEY: &str = "abcde";
    const INPUT: &str = "123455";
    const EXPECT: &[u8] = &[80, 80, 80, 80, 80, 84];

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
}

mod key_len_equal_to_data {

    use super::*;

    const KEY: &str = "The quick brown fox jumped over the lazy dog.";
    const INPUT: &str = "Wait, oops, this is not the pangram exercise!";
    const EXPECT: &[u8] = &[
        3, 9, 12, 84, 93, 85, 6, 12, 27, 83, 78, 82, 27, 31, 7, 83, 70, 6, 11, 0, 4, 26, 25, 80,
        17, 12, 69, 79, 6, 4, 28, 71, 6, 9, 8, 0, 9, 25, 31, 11, 67, 13, 28, 2, 15,
    ];

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
}

mod key_longer_than_data {

    use super::*;

    const KEY: &str = "A properly cryptographically random key longer than the data can actually be fairly secure.";
    const INPUT: &str = "Text is not cryptographically random.";
    const EXPECT: &[u8] = &[
        21, 69, 8, 6, 79, 25, 22, 82, 2, 22, 84, 67, 17, 11, 9, 4, 27, 8, 21, 19, 17, 24, 1, 10, 2,
        13, 0, 21, 89, 82, 19, 15, 10, 11, 2, 77, 69,
    ];

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
}

mod shakespearean {

    use super::*;

    const KEY: &str = "Forsooth, let us never break our trust!";
    const INPUT: &str =
        "The sacred brothership in which we share shall never from our hearts be lost.";
    const EXPECT: &[u8] = &[
        18, 7, 23, 83, 28, 14, 23, 26, 73, 68, 76, 7, 6, 79, 1, 27, 69, 28, 22, 30, 12, 2, 0, 11,
        28, 69, 22, 3, 73, 12, 29, 82, 87, 17, 82, 6, 27, 21, 83, 35, 79, 1, 27, 14, 3, 24, 72, 66,
        69, 26, 0, 6, 0, 19, 1, 79, 3, 69, 25, 16, 0, 0, 10, 23, 4, 19, 31, 83, 79, 23, 23, 0, 24,
        29, 6, 7, 90,
    ];

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
}

mod comics {

    use super::*;

    const KEY: &str = "Who knows what evil lurks in the hearts of men?";
    const INPUT: &str =
        "Spiderman! It's spiderman! Not a bird, or a plane, or a fireman! Just spiderman!";
    const EXPECT: &[u8] = &[
        4, 24, 6, 68, 14, 28, 2, 22, 29, 1, 87, 33, 21, 83, 83, 69, 5, 25, 5, 68, 9, 7, 31, 10, 29,
        1, 73, 32, 79, 0, 72, 4, 0, 10, 12, 19, 22, 88, 83, 79, 29, 70, 65, 77, 21, 2, 94, 57, 13,
        67, 0, 4, 28, 79, 22, 83, 70, 30, 26, 4, 25, 65, 11, 87, 73, 38, 85, 31, 1, 82, 24, 3, 73,
        13, 11, 82, 25, 9, 11, 1,
    ];

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
}

mod mad_science {

    use super::*;

    const KEY: &str = "TRANSMUTATION_NOTES_1";
    const INPUT: &str = "If wishes were horses, beggars would ride.";
    const EXPECT: &[u8] = &[
        29, 52, 97, 57, 58, 62, 61, 49, 50, 116, 62, 42, 60, 58, 110, 39, 59, 55, 32, 58, 66, 120,
        114, 35, 43, 52, 42, 52, 38, 50, 116, 62, 32, 59, 51, 42, 111, 38, 44, 55, 58, 31,
    ];

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
}

mod metaphor {

    use super::*;

    const KEY: &str = "Contextualism";
    const INPUT: &str = "The globe is text, its people prose; all the world's a page.";
    const EXPECT: &[u8] = &[
        23, 7, 11, 84, 2, 20, 27, 23, 4, 76, 0, 0, 77, 55, 10, 22, 0, 73, 88, 29, 1, 18, 76, 25,
        22, 2, 51, 3, 11, 84, 21, 10, 27, 6, 4, 87, 73, 18, 1, 47, 79, 26, 28, 0, 88, 3, 26, 19, 0,
        13, 84, 30, 99, 14, 78, 4, 4, 31, 17, 91,
    ];

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
}

mod emoji {

    use super::*;

    const KEY: &str = "🔑🗝️… 🎹?";
    const INPUT: &str = "⌨️! 🔒+💻+🧠=🔓";
    const EXPECT: &[u8] = &[
        18, 19, 60, 126, 72, 16, 182, 189, 31, 39, 27, 112, 171, 86, 191, 98, 36, 165, 73, 160, 87,
        63, 169, 97, 111, 11, 4,
    ];

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
}
