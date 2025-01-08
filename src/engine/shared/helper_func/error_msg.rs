use failure::Fail;

/// Sometimes, bad stuff happens.
#[derive(Clone, Debug, Fail)]
pub enum Error {
    /// The FEN string is invalid
    #[fail(display = "Invalid FEN string: {}", fen)]
    InvalidFen { fen: String },
}
