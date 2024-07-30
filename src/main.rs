use support::Dispatch;

mod balances; // import the module
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
}

pub enum RuntimeCall {
    // ao tranferir um valor para outra conta, não preciso passar o 'from',
    // pois o nosso self.dispatch já sabe quem é o 'from', ou seja,
    // quem está transferindo. Portanto, basta informamos o 'to' e o 'amount'
    // esse `BalancesTranfer` será invocado no support::DispachResult{....}
    BalancesTranfer {
        to: types::AccountId,
        amount: types::Amount,
    },
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

/// coordena as ações entre o Balances e o System, ou outros módulos existentes
#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Runtime>,
    system: system::Pallet<Runtime>, // aqui estamos passando o nosso Runtime que implementa o config do system.rs
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // dispach a call on behalf od a caller. Increments the caller's nonce.
    // Dispach allows us to edentify with undelying modules call we want to execute.
    // Note that we extract the 'caller' from extrinsic and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller, // referência ao Caller do support.rs
        runtime_call: Self::Call, // referência ao Caller do support.rs
    ) -> support::DispachResult {
        // o que o user da blockchain está tentando fazer?
        // é um `RuntimeCall::BalancesTranfer`?
        match runtime_call {
            // Sim. Então vamos rotear essa chamada para o Pallet Balances,
            // enviando os três argumentos: `caller`, `to` e `amount`
            RuntimeCall::BalancesTranfer { to, amount } => {
                self.balances.transfer(caller, to, amount)?;
            }
        }

        Ok(())
    }
}

// implementa a interface Runtime
impl Runtime {
    // instancia o 'balances' e o 'system'
    pub fn new() -> Self {
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

    /// execute a block of extrinsics.
    fn execute_block(&mut self, block: types::Block) -> support::DispachResult {
        // incrementamos o número do bloco
        self.system.increment_block_number();

        // verificamos se o número do block que está vindo é igual
        // ao número do bloco atual.
        // Por exemplo se estamos tentando executar o bloco número 5 e
        // o bloco atual é 4 ou 6, não pode prosseguir
        if self.system.get_block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        // percorremos o `block.extrinsic` que é um vetor,
        // e para cada laço extraimos o `caller` e o `call`, que é o tipo de evento
        // o `caller` deseja fazer na blockchain
        for (counter, support::Extrinsic { caller, call }) in
            block.extrinsic.into_iter().enumerate()
        {
            // incrementamos o nonce do caller
            self.system.increment_nonce(&caller);

            // chama o método dispatch do Runtime, 
            // passando o caller (quem está iniciando a transação) 
            // e o call (a ação que deve ser executada).
            let _ = self.dispatch(caller, call).map_err(|e| {
                
                // O .map_err(|e| { ... }) é usado para tratar 
                // qualquer erro que possa ocorrer durante o dispatch. 
                // Se ocorrer um erro, o código dentro dessa closure será executado.
                // Dentro da closure, temos um eprintln! que imprime uma mensagem de erro formatada. 
                // Esta mensagem inclui:
                // 1. O número do bloco atual (block.header.block_number)
                // 2. O número da transação dentro do bloco (counter)
                // 3. A mensagem de erro específica (e)

                // Esta abordagem permite que o sistema 
                // continue processando as próximas transações do bloco, 
                // mesmo se uma transação específica falhar, 
                // apenas registrando o erro para referência futura.
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsict Number: {}\n\tError: {}",
                    block.header.block_number, counter, e
                )
            });
        }

        Ok(())
    }
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
    let julio: String = "julio".to_string();

    // definimos os saldos para miriam no valor de 10.000
    runtime.balances.set_balance(&miriam, 10000);

    // preparando o bloco 1	
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsic: vec![
            support::Extrinsic {
                caller: miriam.clone(),
                call: RuntimeCall::BalancesTranfer { to: lucio.clone(), amount: 5000 }
            },
            support::Extrinsic {
                caller: miriam.clone(),
                call: RuntimeCall::BalancesTranfer { to: julio.clone(), amount: 1000 }
            },
            support::Extrinsic {
                caller: lucio.clone(),
                call: RuntimeCall::BalancesTranfer { to: miriam.clone(), amount: 3000 }
            },
        ],
    };

    // preparando o bloco 2	
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsic: vec![
            support::Extrinsic {
                caller: miriam.clone(),
                call: RuntimeCall::BalancesTranfer { to: lucio.clone(), amount: 5000 }
            },
            support::Extrinsic {
                caller: miriam.clone(),
                call: RuntimeCall::BalancesTranfer { to: julio.clone(), amount: 1000 }
            },
            support::Extrinsic {
                caller: lucio.clone(),
                call: RuntimeCall::BalancesTranfer { to: miriam, amount: 3000 }
            },
        ],
    };

    // executamos a transação
    let _ = runtime.execute_block(block_1).expect("Failed to execute block 1");
    let _ = runtime.execute_block(block_2).expect("Failed to execute block 2");

    // exibo que há dentro do runtime
    println!("{:#?}", runtime)
}
