pub struct Import {
  pub module:      String,
  pub name:        String,
  pub description: ImportDescription,
}

pub enum ImportDescription {
  Func(u32),
  // Table(),
  // Mem(),
  // Global(),
}
