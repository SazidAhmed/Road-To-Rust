// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize

// signed variant can store numbers from -(2n - 1) to 2n - 1 - 1
// i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127.

// Unsigned variants can store numbers from 0 to 2n - 1
// u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.