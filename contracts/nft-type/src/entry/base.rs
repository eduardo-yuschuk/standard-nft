#[allow(unused_imports)]
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, packed::*, prelude::*},
    high_level::{load_cell_data, load_script},
};
#[allow(unused_imports)]
use script_utils::{
    class::{Class, CLASS_TYPE_ARGS_LEN},
    error::Error,
    helper::{
        check_group_input_witness_is_none_with_type, count_cells_by_type, load_cell_data_by_type,
        load_class_type, load_output_type_args_ids, Action,
    },
    nft::{Nft, NFT_TYPE_ARGS_LEN},
};

#[allow(dead_code)]
pub struct Base {}
impl Base {
    #[allow(dead_code)]
    pub fn handle_creation(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
    #[allow(dead_code)]
    pub fn handle_update(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
    #[allow(dead_code)]
    pub fn handle_destroying(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
}