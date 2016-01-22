mod author;
mod book;
pub mod memoryprovider;

pub mod model {

pub use model::author::Author;
pub use model::author::Date;
pub use model::book::Book;
pub use model::book::BookProvider;
// pub use model::memoryprovider::MemoryProvider;
pub use model::book::Metadata;

}
