multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{storage, zombie_factory};
use crypto_kitties_proxy::Kitty;

mod crypto_kitties_proxy {
  multiversx_sc::imports!();
  multiversx_sc::derive_imports!();

  #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
  pub struct Kitty {
    pub is_gestating: bool,
    pub is_ready: bool,
    pub cooldown_index: u64,
    pub next_action_at: u64,
    pub siring_with_id: u64,
    pub birth_time: u64,
    pub matron_id: u64,
    pub sire_id: u64,
    pub generation: u64,
    pub genes: u64,
  }

  #[multiversx_sc::proxy]
  pub trait CryptoKitties {
    #[endpoint]
    fn get_kitty(&self, id: usize) -> Kitty;
  }
}

#[multiversx_sc::module]
pub trait ZombieFeeding:
  storage::Storage
  + zombie_factory::ZombieFactory
{
  #[endpoint]
  fn feed_and_multiply(&self, zombie_id: usize, target_dna: u64, species: ManagedBuffer) {
    let caller = self.blockchain().get_caller();
    require!(
        caller == self.zombie_owner(&zombie_id).get(),
        "Only the owner of the zombie can perform this operation"
    );
    let my_zombie = self.zombies(&zombie_id).get();
    let dna_digits = self.dna_digits().get();
    let max_dna_value = u64::pow(10u64, dna_digits as u32);
    let verified_target_dna = target_dna % max_dna_value;
    let mut new_dna = (my_zombie.dna + verified_target_dna) / 2;

    if species == ManagedBuffer::from(b"kitty") {
        new_dna = new_dna - new_dna % 100 + 99;
    }

    self.create_zombie(caller, ManagedBuffer::from(b"NoName"), new_dna);
  }

  #[callback]
  fn get_kitty_callback(
    &self,
    #[call_result] result: ManagedAsyncCallResult<Kitty>,
    zombie_id: usize,
  ) {
    match result {
      ManagedAsyncCallResult::Ok(kitty) => {
        let kitty_dna = kitty.genes;
        self.feed_and_multiply(zombie_id, kitty_dna, ManagedBuffer::from(b"kitty"));
      },
      ManagedAsyncCallResult::Err(_) => {},
    }
  }

  #[endpoint]
  fn feed_on_kitty(
    &self,
    zombie_id: usize,
    kitty_id: usize,
  ) {
    let crypto_kitties_sc_address = self.crypto_kitties_sc_address().get();
    self.kitty_proxy(crypto_kitties_sc_address)
      .get_kitty(kitty_id)
      .async_call()
      .with_callback(self.callbacks().get_kitty_callback(zombie_id))
      .call_and_exit();
  }

  #[proxy]
  fn kitty_proxy(&self, to: ManagedAddress) -> crypto_kitties_proxy::Proxy<Self::Api>;
}
