
use crate::script::Script;
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TxOutput {
    prev_tx: Vec<u8>,
    prev_index: Vec<u8>,
    script_sig: Script,
    sequence: Vec<u8>,
}
