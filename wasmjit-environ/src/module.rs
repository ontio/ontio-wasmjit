//! Data structures for representing decoded wasm modules.

use crate::tunables::Tunables;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use cranelift_codegen::ir;
use cranelift_entity::{EntityRef, PrimaryMap};
use cranelift_wasm::{
    DefinedFuncIndex, DefinedGlobalIndex, DefinedMemoryIndex, DefinedTableIndex, FuncIndex, Global,
    GlobalIndex, Memory, MemoryIndex, SignatureIndex, Table, TableIndex,
};

/// A WebAssembly table initializer.
#[derive(Clone, Debug, Hash)]
pub struct TableElements {
    /// The index of a table to initialize.
    pub table_index: TableIndex,
    /// Optionally, a global variable giving a base index.
    pub base: Option<GlobalIndex>,
    /// The offset to add to the base.
    pub offset: usize,
    /// The values to write into the table elements.
    pub elements: Box<[FuncIndex]>,
}

/// Implemenation styles for WebAssembly linear memory.
#[derive(Debug, Clone, Hash)]
pub enum MemoryStyle {
    /// The actual memory can be resized and moved.
    Dynamic,
    /// Addresss space is allocated up front.
    Static {
        /// The number of mapped and unmapped pages.
        bound: u32,
    },
}

impl MemoryStyle {
    /// Decide on an implementation style for the given `Memory`.
    pub fn for_memory(memory: Memory, tunables: &Tunables) -> (Self, u64) {
        if let Some(maximum) = memory.maximum {
            if maximum <= tunables.static_memory_bound {
                // A heap with a declared maximum can be immovable, so make
                // it static.
                assert!(tunables.static_memory_bound >= memory.minimum);
                return (
                    Self::Static {
                        bound: tunables.static_memory_bound,
                    },
                    tunables.static_memory_offset_guard_size,
                );
            }
        }

        // Otherwise, make it dynamic.
        (Self::Dynamic, tunables.dynamic_memory_offset_guard_size)
    }
}

/// A WebAssembly linear memory description along with our chosen style for
/// implementing it.
#[derive(Debug, Clone, Hash)]
pub struct MemoryPlan {
    /// The WebAssembly linear memory description.
    pub memory: Memory,
    /// Our chosen implementation style.
    pub style: MemoryStyle,
    /// Our chosen offset-guard size.
    pub offset_guard_size: u64,
}

impl MemoryPlan {
    /// Draw up a plan for implementing a `Memory`.
    pub fn for_memory(memory: Memory, tunables: &Tunables) -> Self {
        let (style, offset_guard_size) = MemoryStyle::for_memory(memory, tunables);
        Self {
            memory,
            style,
            offset_guard_size,
        }
    }
}

/// A translated WebAssembly module, excluding the function bodies and
/// memory initializers.
#[derive(Debug)]
pub struct Module {
    /// Unprocessed signatures exactly as provided by `declare_signature()`.
    pub signatures: PrimaryMap<SignatureIndex, ir::Signature>,

    /// Names of imported functions.
    pub imported_funcs: PrimaryMap<FuncIndex, (String, String)>,

    /// Types of functions, imported and local.
    pub functions: PrimaryMap<FuncIndex, SignatureIndex>,

    /// WebAssembly tables.
    pub tables: PrimaryMap<TableIndex, Table>,

    /// WebAssembly linear memory plans.
    pub memory_plans: PrimaryMap<MemoryIndex, MemoryPlan>,

    /// WebAssembly global variables.
    pub globals: PrimaryMap<GlobalIndex, Global>,

    /// Exported invoke func.
    pub invoke_func: Option<FuncIndex>,

    /// WebAssembly table initializers.
    pub table_elements: Vec<TableElements>,
}

impl Module {
    /// Allocates the module data structures.
    pub fn new() -> Self {
        Self {
            signatures: PrimaryMap::new(),
            imported_funcs: PrimaryMap::new(),
            functions: PrimaryMap::new(),
            tables: PrimaryMap::new(),
            memory_plans: PrimaryMap::new(),
            globals: PrimaryMap::new(),
            invoke_func: None,
            table_elements: Vec::new(),
        }
    }

    /// Convert a `DefinedFuncIndex` into a `FuncIndex`.
    pub fn func_index(&self, defined_func: DefinedFuncIndex) -> FuncIndex {
        FuncIndex::new(self.imported_funcs.len() + defined_func.index())
    }

    /// Convert a `FuncIndex` into a `DefinedFuncIndex`. Returns None if the
    /// index is an imported function.
    pub fn defined_func_index(&self, func: FuncIndex) -> Option<DefinedFuncIndex> {
        if func.index() < self.imported_funcs.len() {
            None
        } else {
            Some(DefinedFuncIndex::new(
                func.index() - self.imported_funcs.len(),
            ))
        }
    }

    /// Test whether the given function index is for an imported function.
    pub fn is_imported_function(&self, index: FuncIndex) -> bool {
        index.index() < self.imported_funcs.len()
    }

    /// Convert a `DefinedTableIndex` into a `TableIndex`.
    pub fn table_index(&self, defined_table: DefinedTableIndex) -> TableIndex {
        TableIndex::new(defined_table.index())
    }

    /// Convert a `TableIndex` into a `DefinedTableIndex`. Returns None if the
    /// index is an imported table.
    pub fn defined_table_index(&self, table: TableIndex) -> DefinedTableIndex {
        DefinedTableIndex::new(table.index())
    }

    /// Convert a `DefinedMemoryIndex` into a `MemoryIndex`.
    pub fn memory_index(&self, defined_memory: DefinedMemoryIndex) -> MemoryIndex {
        MemoryIndex::new(defined_memory.index())
    }

    /// Convert a `MemoryIndex` into a `DefinedMemoryIndex`. Returns None if the
    /// index is an imported memory.
    pub fn defined_memory_index(&self, memory: MemoryIndex) -> DefinedMemoryIndex {
        DefinedMemoryIndex::new(memory.index())
    }

    /// Convert a `DefinedGlobalIndex` into a `GlobalIndex`.
    pub fn global_index(&self, defined_global: DefinedGlobalIndex) -> GlobalIndex {
        GlobalIndex::new(defined_global.index())
    }

    /// Convert a `GlobalIndex` into a `DefinedGlobalIndex`. Returns None if the
    /// index is an imported global.
    pub fn defined_global_index(&self, global: GlobalIndex) -> DefinedGlobalIndex {
        DefinedGlobalIndex::new(global.index())
    }
}
