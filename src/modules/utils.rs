/// A list of module subsections in the Application.
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

/// A list of subblocks of module subsections in the Application.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SubSubBlock {
    None,
    TodoTitle,
    TodoCreatedAt,
    TodoCategory,
    TodoLabel,
    TodoStatus,
    TodoDescription,
    TodoMilestones,
}

impl SubSubBlock {
    /// Convert SubSubBlock into a integer
    pub fn from(items: SubSubBlock) -> usize {
        match items {
            SubSubBlock::None => 0,
            SubSubBlock::TodoTitle => 1,
            SubSubBlock::TodoCreatedAt => 2,
            SubSubBlock::TodoCategory => 3,
            SubSubBlock::TodoLabel => 4,
            SubSubBlock::TodoStatus => 5,
            SubSubBlock::TodoDescription => 6,
            SubSubBlock::TodoMilestones => 7,
        }
    }
    /// Convert integer into a SubSubBlock
    pub fn from_usize(value: usize) -> SubSubBlock {
        match value {
            0 => SubSubBlock::None,
            1 => SubSubBlock::TodoTitle,
            2 => SubSubBlock::TodoCreatedAt,
            3 => SubSubBlock::TodoCategory,
            4 => SubSubBlock::TodoLabel,
            5 => SubSubBlock::TodoStatus,
            6=> SubSubBlock::TodoDescription,
            7=>SubSubBlock::TodoMilestones,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

/// Current Sub Sub Block Focus in the Application
#[derive(Debug)]
pub struct SubSubBlockControl {
    pub focus_block: SubSubBlock,
    pub subsubblock_active: bool,
}

impl Default for SubSubBlockControl {
    fn default() -> Self {
        Self {
            focus_block: SubSubBlock::None,
            subsubblock_active: false,
        }
    }
}

impl SubSubBlockControl {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
}