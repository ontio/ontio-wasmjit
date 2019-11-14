//! An `Instance` contains all the runtime state used by execution of a
//! wasm module (except its callstack and register state). An
//! `InstanceHandle` is a reference-counting handle for an `Instance`.

use crate::export::ExportFunc;
use crate::memory::LinearMemory;
use crate::mmap::Mmap;
use crate::signalhandlers::{wasmjit_init_eager, wasmjit_init_finish};
use crate::table::Table;
use crate::vmcontext::{
    VMBuiltinFunctionsArray, VMCallerCheckedAnyfunc, VMContext, VMFunctionBody, VMFunctionImport,
    VMGlobalDefinition, VMMemoryDefinition, VMSharedSignatureIndex, VMTableDefinition,
};
use crate::SignatureRegistry;
use core::any::Any;
use core::convert::TryFrom;
use core::slice;
use core::{mem, ptr};
use cranelift_codegen::ir;
use cranelift_entity::EntityRef;
use cranelift_entity::{BoxedSlice, PrimaryMap};
use cranelift_wasm::{
    DefinedFuncIndex, DefinedGlobalIndex, DefinedMemoryIndex, DefinedTableIndex, FuncIndex,
    GlobalInit, SignatureIndex,
};
use indexmap;
use ontio_wasmjit_environ::{DataInitializer, Module, TableElements, VMOffsets};
use std::borrow::ToOwned;
use std::boxed::Box;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

fn signature_id(
    vmctx: &VMContext,
    offsets: &VMOffsets,
    index: SignatureIndex,
) -> VMSharedSignatureIndex {
    #[allow(clippy::cast_ptr_alignment)]
    unsafe {
        let ptr = (vmctx as *const VMContext as *const u8)
            .add(usize::try_from(offsets.vmctx_vmshared_signature_id(index)).unwrap());
        *(ptr as *const VMSharedSignatureIndex)
    }
}

fn imported_function<'vmctx>(
    vmctx: &'vmctx VMContext,
    offsets: &VMOffsets,
    index: FuncIndex,
) -> &'vmctx VMFunctionImport {
    #[allow(clippy::cast_ptr_alignment)]
    unsafe {
        let ptr = (vmctx as *const VMContext as *const u8)
            .add(usize::try_from(offsets.vmctx_vmfunction_import(index)).unwrap());
        &*(ptr as *const VMFunctionImport)
    }
}

fn table<'vmctx>(
    vmctx: &'vmctx VMContext,
    offsets: &VMOffsets,
    index: DefinedTableIndex,
) -> &'vmctx VMTableDefinition {
    unsafe {
        let ptr = (vmctx as *const VMContext as *const u8)
            .add(usize::try_from(offsets.vmctx_vmtable_definition(index)).unwrap());
        #[allow(clippy::cast_ptr_alignment)]
        &*(ptr as *const VMTableDefinition)
    }
}

fn table_mut<'vmctx>(
    vmctx: &'vmctx mut VMContext,
    offsets: &VMOffsets,
    index: DefinedTableIndex,
) -> &'vmctx mut VMTableDefinition {
    unsafe {
        let ptr = (vmctx as *mut VMContext as *mut u8)
            .add(usize::try_from(offsets.vmctx_vmtable_definition(index)).unwrap());
        #[allow(clippy::cast_ptr_alignment)]
        &mut *(ptr as *mut VMTableDefinition)
    }
}

fn memory<'vmctx>(
    vmctx: &'vmctx VMContext,
    offsets: &VMOffsets,
    index: DefinedMemoryIndex,
) -> &'vmctx VMMemoryDefinition {
    unsafe {
        let ptr = (vmctx as *const VMContext as *const u8)
            .add(usize::try_from(offsets.vmctx_vmmemory_definition(index)).unwrap());
        #[allow(clippy::cast_ptr_alignment)]
        &*(ptr as *const VMMemoryDefinition)
    }
}

fn memory_mut<'vmctx>(
    vmctx: &'vmctx mut VMContext,
    offsets: &VMOffsets,
    index: DefinedMemoryIndex,
) -> &'vmctx mut VMMemoryDefinition {
    unsafe {
        let ptr = (vmctx as *mut VMContext as *mut u8)
            .add(usize::try_from(offsets.vmctx_vmmemory_definition(index)).unwrap());
        #[allow(clippy::cast_ptr_alignment)]
        &mut *(ptr as *mut VMMemoryDefinition)
    }
}

fn global<'vmctx>(
    vmctx: &'vmctx VMContext,
    offsets: &VMOffsets,
    index: DefinedGlobalIndex,
) -> &'vmctx VMGlobalDefinition {
    unsafe {
        let ptr = (vmctx as *const VMContext as *const u8)
            .add(usize::try_from(offsets.vmctx_vmglobal_definition(index)).unwrap());
        #[allow(clippy::cast_ptr_alignment)]
        &*(ptr as *const VMGlobalDefinition)
    }
}

fn global_mut<'vmctx>(
    vmctx: &'vmctx mut VMContext,
    offsets: &VMOffsets,
    index: DefinedGlobalIndex,
) -> &'vmctx mut VMGlobalDefinition {
    unsafe {
        let ptr = (vmctx as *mut VMContext as *mut u8)
            .add(usize::try_from(offsets.vmctx_vmglobal_definition(index)).unwrap());
        #[allow(clippy::cast_ptr_alignment)]
        &mut *(ptr as *mut VMGlobalDefinition)
    }
}

/// A WebAssembly instance.
///
/// This is repr(C) to ensure that the vmctx field is last.
#[repr(C)]
pub struct Instance {
    /// The underlying mmap that holds this `Instance`.
    mmap: Mmap,

    /// The `Module` this `Instance` was instantiated from.
    module: Rc<Module>,

    /// Offsets in the `vmctx` region.
    offsets: VMOffsets,

    /// WebAssembly linear memory data.
    memories: BoxedSlice<DefinedMemoryIndex, LinearMemory>,

    /// WebAssembly table data.
    tables: BoxedSlice<DefinedTableIndex, Table>,

    /// Pointers to functions in executable memory.
    finished_functions: BoxedSlice<DefinedFuncIndex, *const VMFunctionBody>,

    /// Available gas left.
    pub(crate) gas_left: Arc<AtomicU64>,

    /// Hosts can store arbitrary per-instance information here.
    host_state: Box<dyn Any>,

    /// Additional context used by compiled wasm code. This field is last, and
    /// represents a dynamically-sized array that extends beyond the nominal
    /// end of the struct (similar to a flexible array member).
    vmctx: VMContext,
}

#[allow(clippy::cast_ptr_alignment)]
impl Instance {
    /// Return the indexed `VMSharedSignatureIndex`.
    #[allow(dead_code)]
    fn signature_id(&self, index: SignatureIndex) -> VMSharedSignatureIndex {
        signature_id(&self.vmctx, &self.offsets, index)
    }

    /// Return a pointer to the `VMSharedSignatureIndex`s.
    fn signature_ids_ptr(&mut self) -> *mut VMSharedSignatureIndex {
        unsafe {
            (&mut self.vmctx as *mut VMContext as *mut u8)
                .add(usize::try_from(self.offsets.vmctx_signature_ids_begin()).unwrap())
                as *mut VMSharedSignatureIndex
        }
    }

    /// Return a pointer to the `VMFunctionImport`s.
    fn imported_functions_ptr(&mut self) -> *mut VMFunctionImport {
        unsafe {
            (&mut self.vmctx as *mut VMContext as *mut u8)
                .add(usize::try_from(self.offsets.vmctx_imported_functions_begin()).unwrap())
                as *mut VMFunctionImport
        }
    }

    /// Return the indexed `VMTableDefinition`.
    #[allow(dead_code)]
    fn table(&self, index: DefinedTableIndex) -> &VMTableDefinition {
        table(&self.vmctx, &self.offsets, index)
    }

    /// Return the indexed `VMTableDefinition`.
    #[allow(dead_code)]
    fn table_mut(&mut self, index: DefinedTableIndex) -> &mut VMTableDefinition {
        table_mut(&mut self.vmctx, &self.offsets, index)
    }

    /// Return a pointer to the `VMTableDefinition`s.
    fn tables_ptr(&mut self) -> *mut VMTableDefinition {
        unsafe {
            (&mut self.vmctx as *mut VMContext as *mut u8)
                .add(usize::try_from(self.offsets.vmctx_tables_begin()).unwrap())
                as *mut VMTableDefinition
        }
    }

    /// Return the indexed `VMMemoryDefinition`.
    fn memory(&self, index: DefinedMemoryIndex) -> &VMMemoryDefinition {
        memory(&self.vmctx, &self.offsets, index)
    }

    /// Return the indexed `VMMemoryDefinition`.
    fn memory_mut(&mut self, index: DefinedMemoryIndex) -> &mut VMMemoryDefinition {
        memory_mut(&mut self.vmctx, &self.offsets, index)
    }

    /// Return a pointer to the `VMMemoryDefinition`s.
    fn memories_ptr(&mut self) -> *mut VMMemoryDefinition {
        unsafe {
            (&mut self.vmctx as *mut VMContext as *mut u8)
                .add(usize::try_from(self.offsets.vmctx_memories_begin()).unwrap())
                as *mut VMMemoryDefinition
        }
    }

    /// Return the indexed `VMGlobalDefinition`.
    #[allow(dead_code)]
    fn global(&self, index: DefinedGlobalIndex) -> &VMGlobalDefinition {
        global(&self.vmctx, &self.offsets, index)
    }

    /// Return the indexed `VMGlobalDefinition`.
    fn global_mut(&mut self, index: DefinedGlobalIndex) -> &mut VMGlobalDefinition {
        global_mut(&mut self.vmctx, &self.offsets, index)
    }

    /// Return a pointer to the `VMGlobalDefinition`s.
    fn globals_ptr(&mut self) -> *mut VMGlobalDefinition {
        unsafe {
            (&mut self.vmctx as *mut VMContext as *mut u8)
                .add(usize::try_from(self.offsets.vmctx_globals_begin()).unwrap())
                as *mut VMGlobalDefinition
        }
    }

    /// Return a pointer to the `VMBuiltinFunctionsArray`.
    fn builtin_functions_ptr(&mut self) -> *mut VMBuiltinFunctionsArray {
        unsafe {
            (&mut self.vmctx as *mut VMContext as *mut u8)
                .add(usize::try_from(self.offsets.vmctx_builtin_functions_begin()).unwrap())
                as *mut VMBuiltinFunctionsArray
        }
    }

    /// Return a reference to the vmctx used by compiled wasm code.
    pub fn vmctx(&self) -> &VMContext {
        &self.vmctx
    }

    /// Return a raw pointer to the vmctx used by compiled wasm code.
    pub fn vmctx_ptr(&self) -> *const VMContext {
        self.vmctx()
    }

    /// Return a mutable reference to the vmctx used by compiled wasm code.
    pub fn vmctx_mut(&mut self) -> &mut VMContext {
        &mut self.vmctx
    }

    /// Return a mutable raw pointer to the vmctx used by compiled wasm code.
    pub fn vmctx_mut_ptr(&mut self) -> *mut VMContext {
        self.vmctx_mut()
    }

    /// Lookup an export with the given name.
    pub fn lookup(&mut self, field: &str) -> Option<ExportFunc> {
        let export = if let Some(export) = self.module.exports.get(field) {
            export.clone()
        } else {
            return None;
        };
        Some(self.lookup_by_declaration(export))
    }

    /// Lookup an export with the given export declaration.
    pub fn lookup_by_declaration(&mut self, export: FuncIndex) -> ExportFunc {
        lookup_by_declaration(
            &self.module,
            &mut self.vmctx,
            &self.offsets,
            &self.finished_functions,
            export,
        )
    }

    /// Return an iterator over the exports of this instance.
    ///
    /// Specifically, it provides access to the key-value pairs, where they keys
    /// are export names, and the values are export declarations which can be
    /// resolved `lookup_by_declaration`.
    pub fn exports(&self) -> indexmap::map::Iter<String, FuncIndex> {
        self.module.exports.iter()
    }

    /// Return a reference to the custom state attached to this instance.
    pub fn host_state(&mut self) -> &mut dyn Any {
        &mut *self.host_state
    }

    /// Return the offset from the vmctx pointer to its containing Instance.
    pub(crate) fn vmctx_offset() -> isize {
        offset_of!(Self, vmctx) as isize
    }

    /// Return the table index for the given `VMTableDefinition`.
    pub(crate) fn table_index(&self, table: &VMTableDefinition) -> DefinedTableIndex {
        let offsets = &self.offsets;
        let begin = unsafe {
            (&self.vmctx as *const VMContext as *const u8)
                .add(usize::try_from(offsets.vmctx_tables_begin()).unwrap())
        } as *const VMTableDefinition;
        let end: *const VMTableDefinition = table;
        // TODO: Use `offset_from` once it stablizes.
        let index = DefinedTableIndex::new(
            (end as usize - begin as usize) / mem::size_of::<VMTableDefinition>(),
        );
        assert!(index.index() < self.tables.len());
        index
    }

    /// Return the memory index for the given `VMMemoryDefinition`.
    pub(crate) fn memory_index(&self, memory: &VMMemoryDefinition) -> DefinedMemoryIndex {
        let offsets = &self.offsets;
        let begin = unsafe {
            (&self.vmctx as *const VMContext as *const u8)
                .add(usize::try_from(offsets.vmctx_memories_begin()).unwrap())
        } as *const VMMemoryDefinition;
        let end: *const VMMemoryDefinition = memory;
        // TODO: Use `offset_from` once it stablizes.
        let index = DefinedMemoryIndex::new(
            (end as usize - begin as usize) / mem::size_of::<VMMemoryDefinition>(),
        );
        assert!(index.index() < self.memories.len());
        index
    }

    /// Test whether any of the objects inside this instance require signal
    /// handlers to catch out of bounds accesses.
    pub(crate) fn needs_signal_handlers(&self) -> bool {
        self.memories
            .values()
            .any(|memory| memory.needs_signal_handlers)
    }

    /// Return the defined memory as byte slice.
    ///
    /// Panic if `memory_index` is out of bound.
    pub fn memory_slice(&self, memory_index: DefinedMemoryIndex) -> Option<&[u8]> {
        self.memories.get(memory_index).map(|t| t.deref())
    }

    /// Return the defined memory as mutable byte slice.
    ///
    /// Panic if `memory_index` is out of bound.
    pub fn memory_slice_mut(&mut self, memory_index: DefinedMemoryIndex) -> Option<&mut [u8]> {
        self.memories.get_mut(memory_index).map(|t| t.deref_mut())
    }

    /// Grow memory by the specified amount of pages.
    ///
    /// Returns `None` if memory can't be grown by the specified amount
    /// of pages.
    pub(crate) fn memory_grow(
        &mut self,
        memory_index: DefinedMemoryIndex,
        delta: u32,
    ) -> Option<u32> {
        let result = self
            .memories
            .get_mut(memory_index)
            .unwrap_or_else(|| panic!("no memory for index {}", memory_index.index()))
            .grow(delta);

        // Keep current the VMContext pointers used by compiled wasm code.
        *self.memory_mut(memory_index) = self.memories[memory_index].vmmemory();

        result
    }

    /// Returns the number of allocated wasm pages.
    pub(crate) fn memory_size(&mut self, memory_index: DefinedMemoryIndex) -> u32 {
        self.memories
            .get(memory_index)
            .unwrap_or_else(|| panic!("no memory for index {}", memory_index.index()))
            .size()
    }

    /// Grow table by the specified amount of elements.
    ///
    /// Returns `None` if table can't be grown by the specified amount
    /// of elements.
    pub(crate) fn table_grow(&mut self, table_index: DefinedTableIndex, delta: u32) -> Option<u32> {
        let result = self
            .tables
            .get_mut(table_index)
            .unwrap_or_else(|| panic!("no table for index {}", table_index.index()))
            .grow(delta);

        // Keep current the VMContext pointers used by compiled wasm code.
        *self.table_mut(table_index) = self.tables[table_index].vmtable();

        result
    }

    // Get table element by index.
    pub(crate) fn table_get(
        &self,
        table_index: DefinedTableIndex,
        index: u32,
    ) -> Option<&VMCallerCheckedAnyfunc> {
        self.tables
            .get(table_index)
            .unwrap_or_else(|| panic!("no table for index {}", table_index.index()))
            .get(index)
    }

    // Get table mutable element by index.
    pub(crate) fn table_get_mut(
        &mut self,
        table_index: DefinedTableIndex,
        index: u32,
    ) -> Option<&mut VMCallerCheckedAnyfunc> {
        self.tables
            .get_mut(table_index)
            .unwrap_or_else(|| panic!("no table for index {}", table_index.index()))
            .get_mut(index)
    }
}

/// A handle holding an `Instance` of a WebAssembly module.
#[derive(Hash, PartialEq, Eq)]
pub struct InstanceHandle {
    instance: *mut Instance,
}

fn generate_shared_signatures(
    map: &PrimaryMap<SignatureIndex, ir::Signature>,
) -> BoxedSlice<SignatureIndex, VMSharedSignatureIndex> {
    let mut registry = SignatureRegistry::new();
    let mut shared_map = PrimaryMap::new();
    for sig in map.values() {
        let shared = registry.register(sig);
        shared_map.push(shared);
    }

    shared_map.into_boxed_slice()
}

impl InstanceHandle {
    /// Create a new `InstanceHandle` pointing at a new `Instance`.
    pub fn new(
        module: Rc<Module>,
        finished_functions: BoxedSlice<DefinedFuncIndex, *const VMFunctionBody>,
        mut imports: BoxedSlice<FuncIndex, VMFunctionImport>,
        data_initializers: &[DataInitializer<'_>],
        gas_left: Arc<AtomicU64>,
        host_state: Box<dyn Any>,
    ) -> Result<Self, InstantiationError> {
        let mut tables = create_tables(&module);
        let mut memories = create_memories(&module)?;
        let vmshared_signatures = generate_shared_signatures(&module.signatures);

        let vmctx_tables = tables
            .values_mut()
            .map(Table::vmtable)
            .collect::<PrimaryMap<DefinedTableIndex, _>>()
            .into_boxed_slice();

        let vmctx_memories = memories
            .values_mut()
            .map(LinearMemory::vmmemory)
            .collect::<PrimaryMap<DefinedMemoryIndex, _>>()
            .into_boxed_slice();

        let vmctx_globals = create_globals(&module);

        let offsets = VMOffsets::new(mem::size_of::<*const u8>() as u8, &module);

        let mut instance_mmap = Mmap::with_at_least(
            mem::size_of::<Instance>()
                .checked_add(usize::try_from(offsets.size_of_vmctx()).unwrap())
                .unwrap(),
        )
        .map_err(InstantiationError::Resource)?;

        let instance = {
            #[allow(clippy::cast_ptr_alignment)]
            let instance_ptr = instance_mmap.as_mut_ptr() as *mut Instance;
            let instance = Instance {
                mmap: instance_mmap,
                module,
                offsets,
                memories,
                tables,
                finished_functions,
                gas_left,
                host_state,
                vmctx: VMContext { _priv: [] },
            };
            unsafe {
                ptr::write(instance_ptr, instance);
                &mut *instance_ptr
            }
        };

        for func in imports.values_mut() {
            func.vmctx = instance.vmctx_mut_ptr();
        }

        unsafe {
            ptr::copy(
                vmshared_signatures.values().as_slice().as_ptr(),
                instance.signature_ids_ptr() as *mut VMSharedSignatureIndex,
                vmshared_signatures.len(),
            );
            ptr::copy(
                imports.values().as_slice().as_ptr(),
                instance.imported_functions_ptr() as *mut VMFunctionImport,
                imports.len(),
            );
            ptr::copy(
                vmctx_tables.values().as_slice().as_ptr(),
                instance.tables_ptr() as *mut VMTableDefinition,
                vmctx_tables.len(),
            );
            ptr::copy(
                vmctx_memories.values().as_slice().as_ptr(),
                instance.memories_ptr() as *mut VMMemoryDefinition,
                vmctx_memories.len(),
            );
            ptr::copy(
                vmctx_globals.values().as_slice().as_ptr(),
                instance.globals_ptr() as *mut VMGlobalDefinition,
                vmctx_globals.len(),
            );
            ptr::write(
                instance.builtin_functions_ptr() as *mut VMBuiltinFunctionsArray,
                VMBuiltinFunctionsArray::initialized(),
            );
        }

        // Check initializer bounds before initializing anything.
        check_table_init_bounds(instance)?;
        check_memory_init_bounds(instance, data_initializers)?;

        // Apply the initializers.
        initialize_tables(instance)?;
        initialize_memories(instance, data_initializers)?;
        initialize_globals(instance);

        // Ensure that our signal handlers are ready for action.
        // TODO: Move these calls out of `InstanceHandle`.
        wasmjit_init_eager();
        wasmjit_init_finish(instance.vmctx_mut());

        Ok(Self { instance })
    }

    /// Return a reference to the vmctx used by compiled wasm code.
    pub fn vmctx(&self) -> &VMContext {
        self.instance().vmctx()
    }

    /// Return a raw pointer to the vmctx used by compiled wasm code.
    pub fn vmctx_ptr(&self) -> *const VMContext {
        self.instance().vmctx_ptr()
    }

    /// Return a reference-counting pointer to a module.
    pub fn module(&self) -> Rc<Module> {
        self.instance().module.clone()
    }

    /// Return a reference to a module.
    pub fn module_ref(&self) -> &Module {
        &self.instance().module
    }

    /// Return a mutable reference to the vmctx used by compiled wasm code.
    pub fn vmctx_mut(&mut self) -> &mut VMContext {
        self.instance_mut().vmctx_mut()
    }

    /// Return a mutable raw pointer to the vmctx used by compiled wasm code.
    pub fn vmctx_mut_ptr(&mut self) -> *mut VMContext {
        self.instance_mut().vmctx_mut_ptr()
    }

    /// Lookup an export with the given name.
    pub fn lookup(&mut self, field: &str) -> Option<ExportFunc> {
        self.instance_mut().lookup(field)
    }

    /// Lookup an export with the given export declaration.
    pub fn lookup_by_declaration(&mut self, export: FuncIndex) -> ExportFunc {
        self.instance_mut().lookup_by_declaration(export)
    }

    /// Return an iterator over the exports of this instance.
    ///
    /// Specifically, it provides access to the key-value pairs, where they keys
    /// are export names, and the values are export declarations which can be
    /// resolved `lookup_by_declaration`.
    pub fn exports(&self) -> indexmap::map::Iter<String, FuncIndex> {
        self.instance().exports()
    }

    /// Return a reference to the custom state attached to this instance.
    pub fn host_state(&mut self) -> &mut dyn Any {
        self.instance_mut().host_state()
    }

    /// Return the memory index for the given `VMMemoryDefinition` in this instance.
    pub fn memory_index(&self, memory: &VMMemoryDefinition) -> DefinedMemoryIndex {
        self.instance().memory_index(memory)
    }

    /// Grow memory in this instance by the specified amount of pages.
    ///
    /// Returns `None` if memory can't be grown by the specified amount
    /// of pages.
    pub fn memory_grow(&mut self, memory_index: DefinedMemoryIndex, delta: u32) -> Option<u32> {
        self.instance_mut().memory_grow(memory_index, delta)
    }

    /// Return the table index for the given `VMTableDefinition` in this instance.
    pub fn table_index(&self, table: &VMTableDefinition) -> DefinedTableIndex {
        self.instance().table_index(table)
    }

    /// Grow table in this instance by the specified amount of pages.
    ///
    /// Returns `None` if memory can't be grown by the specified amount
    /// of pages.
    pub fn table_grow(&mut self, table_index: DefinedTableIndex, delta: u32) -> Option<u32> {
        self.instance_mut().table_grow(table_index, delta)
    }

    /// Get table element reference.
    ///
    /// Returns `None` if index is out of bounds.
    pub fn table_get(
        &self,
        table_index: DefinedTableIndex,
        index: u32,
    ) -> Option<&VMCallerCheckedAnyfunc> {
        self.instance().table_get(table_index, index)
    }

    /// Get mutable table element reference.
    ///
    /// Returns `None` if index is out of bounds.
    pub fn table_get_mut(
        &mut self,
        table_index: DefinedTableIndex,
        index: u32,
    ) -> Option<&mut VMCallerCheckedAnyfunc> {
        self.instance_mut().table_get_mut(table_index, index)
    }
}

impl InstanceHandle {
    /// Return a reference to the contained `Instance`.
    pub fn instance(&self) -> &Instance {
        unsafe { &*(self.instance as *const Instance) }
    }

    /// Return a mutable reference to the contained `Instance`.
    pub fn instance_mut(&mut self) -> &mut Instance {
        unsafe { &mut *(self.instance as *mut Instance) }
    }
}

impl Drop for InstanceHandle {
    fn drop(&mut self) {
        let instance = self.instance_mut();
        let mmap = mem::replace(&mut instance.mmap, Mmap::new());
        unsafe { ptr::drop_in_place(instance) };
        mem::drop(mmap);
    }
}

fn lookup_by_declaration(
    module: &Module,
    vmctx: &mut VMContext,
    offsets: &VMOffsets,
    finished_functions: &BoxedSlice<DefinedFuncIndex, *const VMFunctionBody>,
    index: FuncIndex,
) -> ExportFunc {
    let signature = module.signatures[module.functions[index]].clone();
    let (address, vmctx) = if let Some(def_index) = module.defined_func_index(index) {
        (finished_functions[def_index], vmctx as *mut VMContext)
    } else {
        let import = imported_function(vmctx, offsets, index);
        (import.body, import.vmctx)
    };
    ExportFunc::new(address, vmctx, signature)
}

fn check_table_init_bounds(instance: &mut Instance) -> Result<(), InstantiationError> {
    let module = Rc::clone(&instance.module);
    for init in &module.table_elements {
        let start = get_table_init_start(init, instance);
        let slice = get_table_slice(init, &instance.module, &mut instance.tables);

        if slice.get_mut(start..start + init.elements.len()).is_none() {
            return Err(InstantiationError::Link(LinkError(
                "elements segment does not fit".to_owned(),
            )));
        }
    }

    Ok(())
}

/// Compute the offset for a memory data initializer.
fn get_memory_init_start(init: &DataInitializer<'_>, instance: &mut Instance) -> usize {
    let mut start = init.location.offset;

    if let Some(base) = init.location.base {
        let global = instance.global_mut(instance.module.defined_global_index(base));
        start += usize::try_from(*unsafe { (*global).as_u32() }).unwrap();
    }

    start
}

/// Return a byte-slice view of a memory's data.
fn get_memory_slice<'instance>(
    init: &DataInitializer<'_>,
    instance: &'instance mut Instance,
) -> &'instance mut [u8] {
    let defined_memory_index = instance
        .module
        .defined_memory_index(init.location.memory_index);
    let memory = instance.memory(defined_memory_index);
    unsafe { slice::from_raw_parts_mut(memory.base, memory.current_length) }
}

fn check_memory_init_bounds(
    instance: &mut Instance,
    data_initializers: &[DataInitializer<'_>],
) -> Result<(), InstantiationError> {
    for init in data_initializers {
        let start = get_memory_init_start(init, instance);
        let mem_slice = get_memory_slice(init, instance);

        if mem_slice.get_mut(start..start + init.data.len()).is_none() {
            return Err(InstantiationError::Link(LinkError(
                "data segment does not fit".to_owned(),
            )));
        }
    }

    Ok(())
}

/// Allocate memory for just the tables of the current module.
fn create_tables(module: &Module) -> BoxedSlice<DefinedTableIndex, Table> {
    let mut tables: PrimaryMap<DefinedTableIndex, _> =
        PrimaryMap::with_capacity(module.tables.len());
    for table in module.tables.values() {
        tables.push(Table::new(table));
    }
    tables.into_boxed_slice()
}

/// Compute the offset for a table element initializer.
fn get_table_init_start(init: &TableElements, instance: &mut Instance) -> usize {
    let mut start = init.offset;

    if let Some(base) = init.base {
        let global = instance.global_mut(instance.module.defined_global_index(base));
        start += usize::try_from(*unsafe { (*global).as_u32() }).unwrap();
    }

    start
}

/// Return a byte-slice view of a table's data.
fn get_table_slice<'instance>(
    init: &TableElements,
    module: &Module,
    tables: &'instance mut BoxedSlice<DefinedTableIndex, Table>,
) -> &'instance mut [VMCallerCheckedAnyfunc] {
    let defined_table_index = module.defined_table_index(init.table_index);
    tables[defined_table_index].as_mut()
}

/// Initialize the table memory from the provided initializers.
fn initialize_tables(instance: &mut Instance) -> Result<(), InstantiationError> {
    let vmctx: *mut VMContext = instance.vmctx_mut();
    let module = Rc::clone(&instance.module);
    for init in &module.table_elements {
        let start = get_table_init_start(init, instance);
        let slice = get_table_slice(init, &instance.module, &mut instance.tables);

        let subslice = &mut slice[start..start + init.elements.len()];
        for (i, func_idx) in init.elements.iter().enumerate() {
            let callee_sig = instance.module.functions[*func_idx];
            let (callee_ptr, callee_vmctx) =
                if let Some(index) = instance.module.defined_func_index(*func_idx) {
                    (instance.finished_functions[index], vmctx)
                } else {
                    let imported_func =
                        imported_function(&instance.vmctx, &instance.offsets, *func_idx);
                    (imported_func.body, imported_func.vmctx)
                };
            let type_index = signature_id(&instance.vmctx, &instance.offsets, callee_sig);
            subslice[i] = VMCallerCheckedAnyfunc {
                func_ptr: callee_ptr,
                type_index,
                vmctx: callee_vmctx,
            };
        }
    }

    Ok(())
}

/// Allocate memory for just the memories of the current module.
fn create_memories(
    module: &Module,
) -> Result<BoxedSlice<DefinedMemoryIndex, LinearMemory>, InstantiationError> {
    let mut memories: PrimaryMap<DefinedMemoryIndex, _> =
        PrimaryMap::with_capacity(module.memory_plans.len());
    for plan in module.memory_plans.values() {
        memories.push(LinearMemory::new(plan).map_err(InstantiationError::Resource)?);
    }
    Ok(memories.into_boxed_slice())
}

/// Initialize the table memory from the provided initializers.
fn initialize_memories(
    instance: &mut Instance,
    data_initializers: &[DataInitializer<'_>],
) -> Result<(), InstantiationError> {
    for init in data_initializers {
        let start = get_memory_init_start(init, instance);
        let mem_slice = get_memory_slice(init, instance);

        let to_init = &mut mem_slice[start..start + init.data.len()];
        to_init.copy_from_slice(init.data);
    }

    Ok(())
}

/// Allocate memory for just the globals of the current module,
/// with initializers applied.
fn create_globals(module: &Module) -> BoxedSlice<DefinedGlobalIndex, VMGlobalDefinition> {
    let mut vmctx_globals = PrimaryMap::with_capacity(module.globals.len());

    for _ in 0..module.globals.values().len() {
        vmctx_globals.push(VMGlobalDefinition::new());
    }

    vmctx_globals.into_boxed_slice()
}

fn initialize_globals(instance: &mut Instance) {
    let module = Rc::clone(&instance.module);
    for (index, global) in module.globals.iter() {
        let def_index = module.defined_global_index(index);
        let to: *mut VMGlobalDefinition = instance.global_mut(def_index);
        match global.initializer {
            GlobalInit::I32Const(x) => *unsafe { (*to).as_i32_mut() } = x,
            GlobalInit::I64Const(x) => *unsafe { (*to).as_i64_mut() } = x,
            GlobalInit::F32Const(x) => *unsafe { (*to).as_f32_bits_mut() } = x,
            GlobalInit::F64Const(x) => *unsafe { (*to).as_f64_bits_mut() } = x,
            GlobalInit::V128Const(x) => *unsafe { (*to).as_u128_bits_mut() } = x.0,
            GlobalInit::GetGlobal(x) => {
                let from = instance.global_mut(module.defined_global_index(x));
                unsafe { *to = *from };
            }
            GlobalInit::Import => panic!("locally-defined global initialized as import"),
        }
    }
}

/// An link error while instantiating a module.
#[derive(Fail, Debug)]
#[fail(display = "Link error: {}", _0)]
pub struct LinkError(pub String);

/// An error while instantiating a module.
#[derive(Fail, Debug)]
pub enum InstantiationError {
    /// Insufficient resources available for execution.
    #[fail(display = "Insufficient resources: {}", _0)]
    Resource(String),

    /// A wasm link error occured.
    #[fail(display = "{}", _0)]
    Link(LinkError),

    /// A compilation error occured.
    #[fail(display = "Trap occurred while invoking start function: {}", _0)]
    StartTrap(String),
}
