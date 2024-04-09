use std::{fmt::Debug, pin::Pin, sync::Arc};

use oxc::{
  allocator::Allocator,
  ast::ast,
  semantic::{Semantic, SemanticBuilder},
  span::SourceType,
};

use crate::{Dummy, StatementExt, TakeIn};

#[allow(clippy::box_collection, clippy::non_send_fields_in_send_ty, unused)]
pub struct OxcAst {
  pub(crate) program: ast::Program<'static>,
  pub(crate) source: Pin<Arc<str>>,
  // Order matters here, we need drop the program first, then drop the allocator. Otherwise, there will be a segmentation fault.
  // The `program` is allocated on the `allocator`. Clippy think it's not used, but it's used.
  pub(crate) allocator: Pin<Box<Allocator>>,
}
impl Debug for OxcAst {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Ast").field("source", &self.source).finish_non_exhaustive()
  }
}

impl Default for OxcAst {
  fn default() -> Self {
    let source = Pin::new(String::default().into());
    let allocator = Box::pin(oxc::allocator::Allocator::default());

    let program = unsafe {
      let alloc = std::mem::transmute::<_, &'static Allocator>(allocator.as_ref());
      ast::Program::dummy(alloc)
    };
    Self { program, source, allocator }
  }
}

unsafe impl Send for OxcAst {}
unsafe impl Sync for OxcAst {}

impl OxcAst {
  pub fn source(&self) -> &str {
    &self.source
  }

  pub fn program(&self) -> &ast::Program<'_> {
    // SAFETY: `&'a ast::Program<'a>` can't outlive the `&'a ast::Program<'static>`.
    unsafe { std::mem::transmute(&self.program) }
  }

  pub fn program_mut(&mut self) -> &mut ast::Program<'_> {
    // SAFETY: `&'a mut ast::Program<'a>` can't outlive the `&'a mut ast::Program<'static>`.
    unsafe { std::mem::transmute(&mut self.program) }
  }

  pub fn program_mut_and_allocator(&mut self) -> (&mut ast::Program<'_>, &Allocator) {
    // SAFETY: `&'a mut ast::Program<'a>` can't outlive the `&'a mut ast::Program<'static>`.
    let program = unsafe { std::mem::transmute(&mut self.program) };
    (program, &self.allocator)
  }

  pub fn make_semantic(&self, ty: SourceType) -> Semantic<'_> {
    let semantic = SemanticBuilder::new(&self.source, ty).build(self.program()).semantic;
    semantic
  }

  // TODO: should move this to `rolldown` crate
  pub fn hoist_import_export_from_stmts(&mut self) {
    let (program, allocator) = self.program_mut_and_allocator();
    let old_body = program.body.take_in(allocator);
    program.body.reserve_exact(old_body.len());
    let mut non_hoisted = oxc::allocator::Vec::new_in(allocator);

    old_body.into_iter().for_each(|top_stmt| {
      if top_stmt.is_module_declaration_with_source() {
        program.body.push(top_stmt);
      } else {
        non_hoisted.push(top_stmt);
      }
    });
    program.body.extend(non_hoisted);
  }
}