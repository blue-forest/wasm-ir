mod export;
pub use export::Export;
mod import;
pub use import::Import;

pub enum ImportExportDescription {
  Func(u32),
  // Table(),
  // Mem(),
  // Global(),
}
