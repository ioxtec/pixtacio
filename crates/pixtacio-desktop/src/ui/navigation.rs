/// Top-level screens. PixtacioApp keeps their instances alive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Image,
    Video,
    Settings,
}
