// Copyright (c) 2013-2014 The Rust Project Developers.
// Copyright (c) 2015-2020 The rust-hex Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Encoding and decoding hex strings.
//!
//! For most cases, you can simply use the [`decode`], [`encode`] and
//! [`encode_upper`] functions. If you need a bit more control, use the traits
//! [`ToHex`] and [`FromHex`] instead.
//!
//! # Example
//!
//! ```
//! # #[cfg(not(feature = "alloc"))]
//! # let mut output = [0; 0x18];
//! #
//! # #[cfg(not(feature = "alloc"))]
//! # hex::encode_to_slice(b"Hello world!", &mut output).unwrap();
//! #
//! # #[cfg(not(feature = "alloc"))]
//! # let hex_string = ::core::str::from_utf8(&output).unwrap();
//! #
//! # #[cfg(feature = "alloc")]
//! let hex_string = hex::encode("Hello world!");
//!
//! println!("{}", hex_string); // Prints "48656c6c6f20776f726c6421"
//!
//! # assert_eq!(hex_string, "48656c6c6f20776f726c6421");
//! ```

#![doc(html_root_url = "https://docs.rs/hex/0.4.3")]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export core functionality
#[doc(inline)]
pub use hex_core::*;

// Re-export macro functionality
#[doc(inline)]
pub use hex_macro::hex;
