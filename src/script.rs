#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Script {
    cmds: Vec<Vec<u8>>,
}