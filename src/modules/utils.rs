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
    TodoSearch,
    TodoListArea,
    TodoAddTask,
    TodoDeleteTask,
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
            SubSubBlock::TodoSearch => 1,
            SubSubBlock::TodoListArea => 2,
            SubSubBlock::TodoAddTask => 3,
            SubSubBlock::TodoDeleteTask => 4,
            SubSubBlock::TodoTitle => 5,
            SubSubBlock::TodoCreatedAt => 6,
            SubSubBlock::TodoCategory => 7,
            SubSubBlock::TodoLabel => 8,
            SubSubBlock::TodoStatus => 9,
            SubSubBlock::TodoDescription => 10,
            SubSubBlock::TodoMilestones => 11,
        }
    }
    /// Convert integer into a SubSubBlock
    pub fn from_usize(value: usize) -> SubSubBlock {
        match value {
            0 => SubSubBlock::None,
            1 => SubSubBlock::TodoSearch,
            2 => SubSubBlock::TodoListArea,
            3 => SubSubBlock::TodoAddTask,
            4 => SubSubBlock::TodoDeleteTask,
            5 => SubSubBlock::TodoTitle,
            6 => SubSubBlock::TodoCreatedAt,
            7 => SubSubBlock::TodoCategory,
            8 => SubSubBlock::TodoLabel,
            9 => SubSubBlock::TodoStatus,
            10 => SubSubBlock::TodoDescription,
            11 => SubSubBlock::TodoMilestones,
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
