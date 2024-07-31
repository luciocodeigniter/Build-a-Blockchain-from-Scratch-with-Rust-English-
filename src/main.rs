use support::Dispatch;

// importando os módulos
mod balances;
mod proof_of_existence;
mod support;
mod system;

// configuramos tipos para serem passados como argumento para os Pallets
mod types {
    use crate::support;

    // definição de tipos
    pub type Amount = u128; // poderia ser o balance aqui também
    pub type AccountId = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;

    // tipos específicos para execução de blocos
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;

    // tipos para Proof Of Existence
    pub type Content = String;
}

// implento o a trait config do system.rs para Runtime
// não posso dar qualquer nome: (RuntimeConfig por exemplo)
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

// implento o a trait config do balances.rs para Runtime
// não posso dar qualquer nome: (RuntimeConfig por exemplo)
impl balances::Config for Runtime {
    type AccountId = types::AccountId;
    type Amount = types::Amount;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

/// Estrutura principal que representa o runtime da blockchain.
/// Este trecho define a estrutura principal do runtime da blockchain.
/// Cada campo representa um módulo (ou "pallet") específico
/// que compõe a funcionalidade da blockchain
/// Cada módulo é parametrizado com <Runtime>,
/// o que significa que eles são configurados especificamente
/// para trabalhar com esta implementação de Runtime.
/// aqui estamos definindo um interface `Runtime`
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {

    /// IMPORTANTE: Aqui dentro tem que ser nessa ordem as propriedades
    /// Essa ordem reflete a hierarquia típica em tempo de execução de blockchain, 
    /// onde o módulo do sistema é fundamental e deve ser inicializado primeiro. 
    /// O módulo de saldos geralmente vem em seguida, 
    /// seguido por outros módulos personalizados como prova_de_existência. 
    /// Esta estrutura garante que a funcionalidade central do sistema esteja 
    /// sempre disponível para outros módulos que possam depender dela

    /// Módulo que lida com funcionalidades básicas do sistema, como contas e blocos
    system: system::Pallet<Runtime>,

    /// Módulo responsável por gerenciar os saldos das contas
    balances: balances::Pallet<Runtime>,

    /// Módulo que implementa a funcionalidade de prova de existência
    proof_of_existence: proof_of_existence::Pallet<Runtime>
}


fn main() {
    // simulando ações na blockchain

    // instanciamos o runtime.
    // esse é genesis state.
    // cada blockchain inicia dessa forma: sem transações
    let mut runtime = Runtime::new();

    // nossos usuários
    let miriam: String = "miriam".to_string();
    let lucio: String = "lucio".to_string();

    // definimos os saldos para miriam no valor de 10.000
    runtime.balances.set_balance(&miriam, 10000);

    // preparando o bloco 1
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },

        // extrinsic precisa receber o `caller` e qual é a chamada `call`
        extrinsic: vec![support::Extrinsic {
            caller: miriam.clone(),
            call: RuntimeCall::balances(balances::Call::transfer {
                to: lucio.clone(),
                amount: 100,
            }),
        }],
    };

    // executamos a transação
    let _ = runtime
        .execute_block(block_1)
        .expect("Failed to execute block 1");

    // preparando o bloco 2 para criação de um `claim`
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsic: vec![support::Extrinsic {
            caller: lucio.clone(),
            call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                claim: "MY_DOC".to_string(),
            }),
        }],
    };

    // executamos a transação
    let _ = runtime
        .execute_block(block_2)
        .expect("Failed to execute block 2");

    // preparando o bloco 3 para remoção de um `claim`
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsic: vec![support::Extrinsic {
            caller: lucio.clone(),
            call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                claim: "MY_DOC".to_string(),
            }),
        }],
    };

    // executamos a transação
    let _ = runtime
        .execute_block(block_3)
        .expect("Failed to execute block 3");

    // preparando o bloco 4 para criação de um `claim`
    let block_4 = types::Block {
        header: support::Header { block_number: 4 },
        extrinsic: vec![support::Extrinsic {
            caller: miriam.clone(),
            call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                claim: "documento_da_miriam".to_string(),
            }),
        }],
    };

    // executamos a transação
    let _ = runtime
        .execute_block(block_4)
        .expect("Failed to execute block 3");

    // exibo que há dentro do runtime
    println!("{:#?}", runtime)
}
