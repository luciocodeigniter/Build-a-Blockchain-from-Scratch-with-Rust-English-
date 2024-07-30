/// the most primitive representation of a block in our blockchain
pub struct Block<Header, Extrinsic> {
    /// the block header contains metadata about the block
    pub header: Header,

    /// the extrinsic represents the state transitions to be executed in this block
    pub extrinsic: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

/// this is as 'extrinsic': literaly an a external message from
/// outside of blockchain. This simplified version of an extrinsic tell us
/// who is making the call and wich call they are making
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

/// The Result type of our runtime, when everything completes successfuly, we return 'Ok(())',
/// otherwise, return a static error message
pub type DispachResult = Result<(), &'static str>;

pub trait Dispatch {
    /// the type used to identify the caller of the function
    type Caller;

    /// the state transition function call the caller is trying to access
    type Call;

    /// This function takes a 'caller' and the 'call' they want to make,
    /// and returns a 'Result' based on the outcome of that function call
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispachResult;
}
