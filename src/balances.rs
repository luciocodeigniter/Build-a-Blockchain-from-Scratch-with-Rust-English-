use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
/**
 * Criamos uma trait para encapsular todos os types que são necessários no Pallet.
 * Isso é muito útil para situações em que precisamos passar muitos types como parâmetros
 *  para os métodos do Pallet. Portanto, passamos apenas o um config que implemente essa trait
 */
pub trait Config {
    // definição de tipos
    type AccountId: Ord + Clone;
    type Amount: Zero + CheckedSub + CheckedAdd + Copy;
}

// Pallet é como se fosse um módulo.
// Também podemos ver o Pallet como uma interface
/**
 * Arquivo responsável por gerenciar os saldos das carteiras dos usuários
 */
#[derive(Debug)] // esse Pallet deriva do Debug para podermos usar o println!
pub struct Pallet<T: Config> {
    // balance precisa ser chave => valor,
    // ou seja, um mapa de string e integer.
    // pode ser o endereço da carteira ou nome e o saldo
    // evidente que num mundo real, os dados são armazenados em banco de dados
    // no nosso caso aqui, estamos armazenando em memória
    balance: BTreeMap<T::AccountId, T::Amount>,
}

/// Tipos de `chamadas` (calls) que esse Pallet provém
pub enum Call<T: Config> {

    // para cada `call` invocada, é necessário informar os respectivos parâmetros ao 
    Transfer { to: T::AccountId, amount: T::Amount },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(caller, to, amount)?;
            }
        }

        Ok(())
    }
}


/**
 * Para a implementação do Pallet, devo passar dois tipos genéricos <AccountId, Amount>,
 * onde cada um deles deve implementar métodos específicos. Vide Where
 */
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        // Aqui podemos criar um novo objeto do tipo Pallet
        // quando quero um novo objeto, basta chamar Pallet::new()
        Pallet {
            balance: BTreeMap::new(),
        }
    }

    // inserimos no map o amount na conta definida.
    // o '&mut self' indica que algo vai mudar entro desse Pallet,
    // ou seja, &mut pemite que read/write
    pub fn set_balance(&mut self, account: &T::AccountId, amount: T::Amount) {
        // Aqui podemos adicionar um novo saldo
        self.balance.insert(account.clone(), amount);
    }

    pub fn get_balance(&self, account: &T::AccountId) -> T::Amount {
        // Aqui podemos pegar o saldo de uma carteira se ela existir,
        // caso contrário retorna zero
        //! note que tem o '*' no início, o que significa que é um
        //! ponteiro para o própria instância de balance (&self)
        *self
            .balance
            .get(&account.clone())
            .unwrap_or(&T::Amount::zero())
    }

    /// Transfere fundos de uma conta para outra.
    ///
    /// # Argumentos
    ///
    /// * `caller: String` - A conta de origem da transferência.
    /// * `to: String` - A conta de destino da transferência.
    /// * `amount: u128` - A quantidade de fundos a ser transferida.
    ///
    /// # Retorno
    ///
    /// Retorna `Result<(), &'static str>`:
    /// - `Ok(())` se a transferência for bem-sucedida
    /// - `Err(&'static str)` com uma mensagem de erro se falhar
    ///
    /// # Exemplos
    ///
    /// ```
    /// let mut balances = Pallet::new();
    /// balances.set_balance(&"Alice".to_string(), 100);
    /// balances.set_balance(&"Bob".to_string(), 50);
    /// let result = balances.transfer("Alice".to_string(), "Bob".to_string(), 30);
    /// assert!(result.is_ok());
    /// ```
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Amount,
    ) -> Result<(), &'static str> {
        // recupero o saldo de quem está querendo transferir
        let caller_balance = self.get_balance(&caller);

        // recupero o saldo para quem vai o 'amount'
        let to_balance = self.get_balance(&to);

        // novo saldo de quem quer fazer a transferência
        // subtraindo o valor do saldo existente.
        // importante é que devemos verificar se o caller_balance
        // tem saldo, caso sim, o resultado é ok, caso contrário
        // lançamos um erro estático: 'Insufficient balance'
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        // novo saldo de quem vai receber o 'amount'
        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding to balance")?;

        // agora atualizamos os saldos
        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        // tudo certo
        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type Amount = u32;
    }

    #[test]
    fn init_balances() {
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();
        balances.set_balance(&"Lucio".to_string(), 100);
        balances.set_balance(&"Miriam".to_string(), 300);
    }

    #[test]
    fn success_tranfer() {
        // instanciamos o Pallet de balances
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        // defino os usuários (account)
        let miriam = "Miriam".to_string();
        let lucio = "Miriam".to_string();

        // definimos os valores iniciais de cada conta
        balances.set_balance(&miriam, 200);
        balances.set_balance(&lucio, 100);

        // '_' para ignorar o retorno do 'transfer'
        // miriam transfere 50 para o lucio
        let _ = balances.transfer(miriam.clone(), lucio.clone(), 50);

        // miriam tem agora 150?
        assert_eq!(balances.get_balance(&miriam), 150);

        // lucio agora tem 150?
        assert_eq!(balances.get_balance(&lucio), 150);
    }

    #[test]
    fn insufficient_balance() {
        // instanciamos o Pallet de balances
        let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

        // defino os usuários (account)
        let caller: String = "Lucio".to_string();
        let to: String = "Miriam".to_string();

        // defino o saldo da miriam para 1500
        balances.set_balance(&caller, 1500);

        // tento transferir 2000 da miriam para o lucio
        let result = balances.transfer(caller.clone(), to.clone(), 2000);

        assert_eq!(result, Err("Insufficient balance"));
    }
}
