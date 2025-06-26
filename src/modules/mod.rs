// This file serves as the root for all modules.
// It re-exports modules for easier access from other parts of the application.

pub mod user;
pub mod conversion;

// Re-export key components for convenience
pub use user::route::configure_routes;
pub use user::crud::UserCRUD;
pub use user::schema::ErrorResponse;

pub use conversion::route::configure_conversion_routes;
