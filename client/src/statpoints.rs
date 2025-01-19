// Constants for stat point allocation
const STATPOINT_ALLOC_QTY: u32 = 10; // Number of stat points to allocate when holding down shift

// Function to allocate stat points
pub fn allocate_stat_points(current_statpoints: u32, is_ctrl_pressed: bool, is_shift_pressed: bool) -> (u32, u32) {
    let mut allocated_points = 0;

    if is_ctrl_pressed {
        allocated_points = current_statpoints; // Allocate all available stat points
    } else if is_shift_pressed {
        allocated_points = STATPOINT_ALLOC_QTY; // Allocate a fixed number of stat points
    } else {
        allocated_points = 1; // Default to allocating one stat point
    }

    let new_statpoints = current_statpoints.saturating_sub(allocated_points); // Prevent underflow
    (new_statpoints, allocated_points) // Return the new stat points and allocated points
}
