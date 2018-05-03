#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rsbx_lib;

use rsbx_lib::sbx_block::Block;

use rsbx_lib::sbx_specs;

fuzz_target!(|data: &[u8]| {
    let mut block = Block::dummy();

    let mut buffer : [u8; 4096] = [0; 4096];

    if data.len() >= 4096 {
        if let Ok(()) = block.sync_from_buffer(data, None) {
            let block_size = sbx_specs::ver_to_block_size(block.get_version());

            block.sync_to_buffer(None, &mut buffer).unwrap();

            assert_eq!(&buffer[0..block_size], &data[0..block_size]);
        }
    }
});
