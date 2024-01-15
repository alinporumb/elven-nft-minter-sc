multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait Storage {
  #[view(getNftTokenId)]
  #[storage_mapper("nftTokenId")]
  fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
