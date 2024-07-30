use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::{collections::BTreeMap, ops::AddAssign};

/**
 * Criamos uma trait para encapsular todos os types que são necessários no Pallet.
 * Isso é muito útil para situações em que precisamos passar muitos types como parâmetros
 *  para os métodos do Pallet. Portanto, passamos apenas o um config que implemente essa trait
 */
pub trait Config {
    // definição de tipos
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + CheckedSub + CheckedAdd + Copy + One + AddAssign;
    type Nonce: Ord + Copy + Zero + One;
}

/**
 * Esse modulo armazena os metadados da nossa blockchain
 */
#[derive(Debug)] // esse Pallet deriva do Debug para podermos usar o println!
pub struct Pallet<T: Config> {
    // T: Config, significa que Pallet depende de um trait que implemente Config
    /// número de blocos que essa blockchain poderá ter 64 elevado a dois
    block_number: T::BlockNumber,

    /// contador de transações que cada usuário (user_wallet_address) já fez na blockchain
    /// <user_wallet_address, counter_of_transactions>
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        // para cada Pallet de system que criamos,
        // o block_number é sempre 0 e o nonce é sempre um map vazio
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn get_block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn get_nonce(&self, account: &T::AccountId) -> T::Nonce {
        *self.nonce.get(account).unwrap_or(&T::Nonce::zero())
    }

    pub fn increment_block_number(&mut self) {
        // dará crash no código se o número ultrapassar o 'u64'
        self.block_number = self
            .get_block_number()
            .checked_add(&T::BlockNumber::one())
            .unwrap();
    }

    pub fn increment_nonce(&mut self, account: &T::AccountId) {
        // se o nonce não existir, o valor é 1,
        let nonce = *self.nonce.get(account).unwrap_or(&T::Nonce::zero()) + T::Nonce::one();
        self.nonce.insert(account.clone(), nonce);
    }
}

#[cfg(test)]
mod test {

    struct TestConfig;

    // Implementando a trait
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        // instanciamos
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();

        // o número de blocos é zero?
        assert_eq!(system.get_block_number(), 0);

        // incrementamos o bloco
        system.increment_block_number();

        // o número de blocos é 1?
        assert_eq!(system.get_block_number(), 1);

        // incrementamos o nonce da Alice
        system.increment_nonce(&"Alice".to_string());

        // o nonce de Alice agora é 1?
        assert_eq!(system.get_nonce(&"Alice".to_string()), 1);
    }
}
