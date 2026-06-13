use crate::allocations::{
    allocation::Allocation, in_memory_allocation::Builder as InMemoryAllocationBuilder,
    stub_allocation::StubAllocation,
};

pub struct AllocationFactory;

impl AllocationFactory {
    pub fn stub() -> Allocation {
        StubAllocation::new().into()
    }
    pub fn in_memory_builder() -> InMemoryAllocationBuilder {
        InMemoryAllocationBuilder::new()
    }
}
