# fountain_raptor_10

Raptor-10 fountain code scheme built on `fountain_engine`.

This crate provides `Raptor10SysCode`, a systematic Raptor-10 code scheme based
on RFC 5053 parameters and generator tables. It is designed to be used with the
generic `Encoder` and `Decoder` types from `fountain_engine`.

## Quick Start

Add the crate to your project. `fountain_utility` is optional, but it provides
the in-memory `VecDataOperater` used in the example below.

```toml
[dependencies]
fountain_raptor_10 = "1.0"
fountain_engine = "1.1"
fountain_utility = "1.0"
```

Create a Raptor-10 systematic code scheme:

```rust
use fountain_engine::CodeScheme;
use fountain_raptor_10::Raptor10SysCode;

let k = 100usize;
let code = Raptor10SysCode::new_with_default_setting(k);
let params = code.get_params();

assert_eq!(params.k, k);
```

Use this `code` with `fountain_engine::Encoder` and `fountain_engine::Decoder`.
The example below shows a complete in-memory encode/decode flow.

## Complete Encode/Decode Example

Load `k` fixed-size source symbols into an encoder, generate coded symbols, then
feed received symbols into a decoder.

Raptor-10 systematic source symbol IDs are `0..k`. Repair symbol IDs start at
`params.num_total()`. The gap between `k` and `params.num_total()` is reserved
for precode symbols and should not be sent as repair symbols.

```rust
use std::collections::HashMap;

use fountain_engine::{CodeScheme, DataOperator, DecodeStatus, Decoder, Encoder};
use fountain_raptor_10::Raptor10SysCode;
use fountain_utility::VecDataOperater;

fn main() {
    let k = 8usize;
    let symbol_size = 4usize;
    let code = Raptor10SysCode::new_with_default_setting(k);
    let params = code.get_params();

    assert_eq!(params.k, k);

    // Your application should split its payload into k equally sized symbols.
    let source_symbols: Vec<Vec<u8>> = (0..k)
        .map(|i| vec![i as u8, i as u8 + 1, i as u8 + 2, i as u8 + 3])
        .collect();

    // Load the source symbols into an in-memory data operator.
    let mut encode_operator = VecDataOperater::new(symbol_size);
    for (source_id, symbol) in source_symbols.iter().enumerate() {
        encode_operator.insert_vector(symbol, source_id);
    }

    // Build an encoder from the Raptor-10 scheme and the data operator.
    let mut encoder = Encoder::new_with_operator(code.clone(), Box::new(encode_operator));
    let mut esi_to_data_id = HashMap::new();

    // Encode the k systematic source symbols.
    for esi in 0..k {
        if let Some(data_id) = encoder.encode_coded_vector(esi) {
            esi_to_data_id.insert(esi, data_id);
        }
    }

    // Encode a few repair symbols. These are useful when some source symbols are lost.
    let first_repair_esi = params.num_total();
    for esi in first_repair_esi..first_repair_esi + 4 {
        if let Some(data_id) = encoder.encode_coded_vector(esi) {
            esi_to_data_id.insert(esi, data_id);
        }
    }

    let encoded_operator = encoder.manager.move_operator();

    // In a real protocol, send each pair `(esi, encoded_symbol)` over the channel.
    // The decoder only needs the ESI and the bytes for each received symbol.
    let mut decoder = Decoder::new_with_operator(
        code,
        Box::new(VecDataOperater::new(symbol_size)),
    );

    let mut decoded = false;
    for esi in 0..k {
        let data_id = esi_to_data_id[&esi];
        let status = decoder.add_coded_vector(esi, encoded_operator.get_vector(data_id));
        if matches!(status, DecodeStatus::Decoded) {
            decoded = true;
            break;
        }
    }

    assert!(decoded);

    let decoded_operator = decoder.manager.move_operator();
    for source_id in 0..k {
        assert_eq!(decoded_operator.get_vector(source_id), source_symbols[source_id]);
    }
}
```

## What It Provides

- `Raptor10SysCode`: a systematic Raptor-10 scheme implementing
  `fountain_engine::traits::CodeScheme`.
- RFC 5053 random, degree, triple, and LT degree-set generators.
- Examples for inspecting the RFC 5053 generator behavior and running local
  encode/decode checks.

## License

MIT License. See [LICENSE](LICENSE).

## Authors

Zigeng Xu and Shenghao Yang.

Copyright (c) 2025, 2026 Zigeng Xu and Shenghao Yang. All rights reserved.
