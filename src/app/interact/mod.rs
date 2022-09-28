pub mod interact;
pub mod drag;

/// Using groups it is easy to have systems only interact with
/// draggables in a specific group.
/// An example usecase would be separate groups for draggables and drop zones.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct Group(pub u8);