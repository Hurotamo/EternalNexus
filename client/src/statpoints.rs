pub const STATPOINT_ALLOC_QTY: u32 = 10; // Number of stat points to allocate when holding down shift
pub const MIN_STATPOINT_ALLOC: u32 = 1; // Minimum number of stat points to allocate

pub fn allocate_stat_points(current_points: u32, allocation: u32) -> Result<u32, String> {
    // Implementation for allocating stat points
    if allocation < MIN_STATPOINT_ALLOC {
        return Err("Allocation must be at least 1".to_string());
    }
    Ok(current_points + allocation)
}

// Additional functions related to stat points can be added here
