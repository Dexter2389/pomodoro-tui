/// A list of sections in the Application.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SubBlock {
    None,
    TodoList,
    TodoDetails,
}

impl From<SubBlock> for usize {
    /// Convert SubBlock into a integer
    fn from(items: SubBlock) -> usize {
        match items {
            SubBlock::None => 0,
            SubBlock::TodoList => 1,
            SubBlock::TodoDetails => 2,
        }
    }
}