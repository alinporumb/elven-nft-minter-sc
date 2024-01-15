const NFT_AMOUNT: u32 = 1;
const TOKEN_NAME: &str = "SPH1";
const ROYALTIES: u32 = 10;
const ATTRIBUTES: &str = "face:face2;head:head3;eyes:eyes4";
const IMAGE_URI: &str =
  "https://bafybeihyksmiahqrqaueetb7b6gzgsu5uxooxolzzlus5h5thb7rknxzby.ipfs.nftstorage.link/1.png";
const METADATA_URI: &str =
  "https://bafybeib35cufpjfc3iyvkb6mreo4ighat7wkiujev7zep3jdlk6johs6a4.ipfs.nftstorage.link/1.json";

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::storage;

#[multiversx_sc::module]
pub trait Operations: storage::Storage {
  #[payable("EGLD")]
  #[endpoint(mint)]
  fn mint(&self, amount_of_tokens: u32) {
    sc_print!("{}", amount_of_tokens);

    require!(self.paused().is_empty(), "The minting is paused or haven't started yet!!!");

    let amount = BigUint::from(NFT_AMOUNT);
    let token = TokenIdentifier::from(TOKEN_NAME);
    let token_name = ManagedBuffer::new_from_bytes(TOKEN_NAME.as_bytes());
    let royalties = BigUint::from(ROYALTIES);
    let attributes = ManagedBuffer::new_from_bytes(ATTRIBUTES.as_bytes());
    let hash_buffer = self.crypto().sha256(&attributes);
    let attributes_hash = hash_buffer.as_managed_buffer();
    let mut uris = ManagedVec::new();
    uris.push(ManagedBuffer::new_from_bytes(IMAGE_URI.as_bytes()));
    uris.push(ManagedBuffer::new_from_bytes(METADATA_URI.as_bytes()));
    let _nonce = self
      .send()
      .esdt_nft_create(
        &token,
        &amount,
        &token_name,
        &royalties,
        &attributes_hash,
        &attributes,
        &uris
      );
  }

  #[only_owner]
  #[payable("EGLD")]
  #[endpoint(issueToken)]
  fn issue_token(
    &self,
    collection_token_name: ManagedBuffer,
    collection_token_ticker: ManagedBuffer,
    is_not_number_in_name: bool,
    nft_token_name: ManagedBuffer,
    token_properties: ManagedBuffer
  ) {
    sc_print!("{}", collection_token_name);
    sc_print!("{}", collection_token_ticker);
    sc_print!("{}", is_not_number_in_name);
    sc_print!("{}", nft_token_name);
    sc_print!("{}", token_properties);

    let issue_cost = self.call_value().egld_value();
    let collection_token_name = ManagedBuffer::new_from_bytes(TOKEN_NAME.as_bytes());
    let collection_token_ticker = ManagedBuffer::new_from_bytes(TOKEN_NAME.as_bytes());
    let properties = NonFungibleTokenProperties {
      can_freeze: false,
      can_wipe: false,
      can_pause: false,
      can_transfer_create_role: false,
      can_change_owner: false,
      can_upgrade: false,
      can_add_special_roles: true,
    };
    self
      .send()
      .esdt_system_sc_proxy()
      .issue_non_fungible(
        issue_cost.clone_value(),
        &collection_token_name,
        &collection_token_ticker,
        properties
      )
      .async_call()
      .with_callback(self.callbacks().issue_callback())
      .call_and_exit();
  }

  #[callback]
  fn issue_callback(
    &self,
    #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>
  ) {
    match result {
      ManagedAsyncCallResult::Ok(token_id) => {
        self.nft_token_id().set(&token_id.unwrap_esdt());
      }
      ManagedAsyncCallResult::Err(_) => {
        // Handle error, e.g., return funds to the owner
      }
    }
  }

  #[only_owner]
  #[endpoint(setLocalRoles)]
  fn set_local_roles(&self, token_roles: ManagedBuffer) {
    sc_print!("{}", token_roles);
    let mut roles: ManagedVec<EsdtLocalRole> = ManagedVec::new();
    roles.push(EsdtLocalRole::NftCreate);
    self
      .send()
      .esdt_system_sc_proxy()
      .set_special_roles(
        &self.blockchain().get_sc_address(),
        &self.nft_token_id().get(),
        roles.iter()
      )
      .async_call()
      .call_and_exit();
  }

  #[only_owner]
  #[endpoint(startMinting)]
  fn start_minting(&self) {
    self.paused().clear();
  }
}
