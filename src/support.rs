/// A representação mais básica de um bloco em nossa blockchain
pub struct Block<Header, Extrinsic> {
    /// O cabeçalho do bloco contém metadados sobre o bloco, como número e hash
    pub header: Header,

    /// As extrinsics representam as transações ou mudanças de estado a serem executadas neste bloco
    pub extrinsic: Vec<Extrinsic>,
}

/// Estrutura que representa o cabeçalho de um bloco
/// Contém informações essenciais sobre o bloco
pub struct Header<BlockNumber> {
    /// O número do bloco, que indica sua posição na cadeia
    pub block_number: BlockNumber,
}

/// Isto é uma 'extrinsic': uma mensagem externa que vem de fora da blockchain.
/// Esta versão simplificada de uma extrinsic nos informa quem está fazendo a chamada
/// e qual chamada está sendo feita
pub struct Extrinsic<Caller, Call> {
    /// O endereço ou identificador de quem está fazendo a chamada
    pub caller: Caller,
    /// A ação ou função que está sendo chamada
    pub call: Call,
}

/// O tipo de resultado do nosso runtime. Quando tudo é concluído com sucesso,
/// retornamos 'Ok(())', caso contrário, retornamos uma mensagem de erro estática
pub type DispachResult = Result<(), &'static str>;

pub trait Dispatch {
    /// O tipo usado para identificar quem está fazendo a chamada
    type Caller;

    /// O tipo que representa a função de transição de estado que o chamador está tentando acessar
    type Call;

    /// Esta função recebe um 'caller' e a 'call' que ele quer fazer,
    /// e retorna um 'Result' baseado no resultado dessa chamada de função.
    /// Ela é responsável por executar a lógica da transação.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispachResult;
}
