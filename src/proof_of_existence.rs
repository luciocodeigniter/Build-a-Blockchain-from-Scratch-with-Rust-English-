use crate::support::DispachResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

/// esse é o módulo Prova de Existência
/// Esse módulo permite aos users afirmar a existência de algum dado, documento, etc
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // Um `Content` pertence a uma `AccountId`,
    // e um `AccountId` por ter diversos `Content`
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            // inicializamos o `claims`
            claims: BTreeMap::new(),
        }
    }

    /// Recupera o owner do claim, se existir, caso contrário retorna null
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(&claim)
    }

    /// Cria um novo claim (content, documento, file, etc) em nome do `Caller`
    /// Retorna um erro se o alguém já criou um `claim` com o mesmo nome
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispachResult {
        match self.get_claim(&claim) {
            // antes de criar um `claim` precisamos verificar se ele já não existe
            Some(_) => Err("Claim already exists"),

            // se não há um `claim` igual ao informado, então inserimos no claims do pallet
            // e retornamos Ok(())
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            }
        }
    }

    /// revoga (abre mão) da existência de algum `claim` (conteúdo)
    /// Essa função só retornará sucesso se o o `caller` for o dono do `claim`
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispachResult {
        // se o `claim` não existir, lançamos um erro
        let claim_owner = self.get_claim(&claim).ok_or("Claim não existe")?;

        // nesse ponto temos um `claim`, mas antes de removê-lo,
        // preciso garantir que o `caller` seja dono dele
        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        // Podemos remover o `claim`
        self.claims.remove(&claim);

        // Tudo certo.
        Ok(())
    }
}

mod tests {

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = String;
    }

    impl crate::system::Config for TestConfig {
        type BlockNumber = u32;
        type AccountId = String;
        type Nonce = u32;
    }

    #[test]
    fn basic_prof_of_existence() {
        // criando uma nova intância do módulo Proof of Existence
        let mut poe = super::Pallet::<TestConfig>::new();

        // ----- Teste em que lucio cria um claim e verificamos se é dono desse claim ---//

        // 1 - criamos o claim
        let _ = poe.create_claim("lucio".to_string(), "my_code".to_string());

        // 2 - verificamos se o Lucio é dono do `claim`
        assert_eq!(
            poe.get_claim(&"my_code".to_string()),
            Some(&"lucio".to_string())
        );

        // --- Teste em que miriam tenta remover um claim que não é dela ---//

        // 1 - verificamos se o erro retornado é "Caller is not the owner of the claim"
        // ao tentamos remover o claim que não é da miriam
        let result = poe.revoke_claim("miriam".to_string(), "my_code".to_string());
        assert_eq!(result, Err("Caller is not the owner of the claim"));

        // --- Teste em que miriam tenta criar um claim que já existe ---//
        let result = poe.create_claim("miriam".to_string(), "my_code".to_string());
        assert_eq!(result, Err("Claim already exists"));


        // --- Teste em que tenta remover um claim que não existe ---//
        let result = poe.revoke_claim("miriam".to_string(), "outro_code".to_string());
        assert_eq!(result, Err("Claim não existe"))
    }
}
