#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod operations;
pub mod storage;

#[multiversx_sc::contract]
pub trait ElvenTools: storage::Storage + operations::Operations {
  #[allow_multiple_var_args]
  #[init]
  fn init(
    &self,
    image_base_cid: ManagedBuffer,
    metadata_base_cid: ManagedBuffer,
    amount_of_tokens: u32,
    tokens_limit_per_address: u32,
    royalties: BigUint,
    selling_price: BigUint,
    file_extension: OptionalValue<ManagedBuffer>,
    tags: OptionalValue<ManagedBuffer>,
    provenance_hash: OptionalValue<ManagedBuffer>,
    is_metadata_in_uris: OptionalValue<bool>
  ) {
    // Use sc_print! to log the values
    sc_print!("{}", image_base_cid);
    sc_print!("{}", metadata_base_cid);
    sc_print!("{}", amount_of_tokens);
    sc_print!("{}", tokens_limit_per_address);
    sc_print!("{}", royalties);
    sc_print!("{}", selling_price);

    if let Some(ext) = file_extension.into_option() {
      sc_print!("{}", ext);
    }

    if let Some(tag) = tags.into_option() {
      sc_print!("{}", tag);
    }

    if let Some(provenance) = provenance_hash.into_option() {
      sc_print!("{}", provenance);
    }

    if let Some(metadata_flag) = is_metadata_in_uris.into_option() {
      sc_print!("{}", metadata_flag);
    }

    let paused = true;
    self.paused().set_if_empty(&paused);
  }

  #[upgrade]
  fn upgrade(&self) {}
}
