/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: 调用者需确保传入的地址是有效的可写内存地址，
    // 且指向一个正确对齐的 u32 类型变量。
    // 这里将 usize 地址转换为裸指针，并通过解引用修改值。
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: 地址保证有效，且指向唯一的 u32 本地变量
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert_eq!(t, 0xAABBCCDD);
    }
}
