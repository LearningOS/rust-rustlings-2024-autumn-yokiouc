/// # Safety
/// 
/// The caller must ensure that the `address` is a valid and mutable reference
/// to a `u32` value. This address must be unique and point to memory that is
/// properly aligned for a `u32`.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller guarantees that the `address` points to a valid,
    // mutable `u32` value. We are dereferencing the raw pointer to modify
    // the value at the memory address.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD; // Modify the value at the given address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}

