#![feature(abi_amdgpu_kernel)]
#![feature(link_llvm_intrinsics)]

// #![feature(lang_items)]
// #![feature(optin_builtin_traits)]

#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "amdgpu-kernel" fn foo(a: *const i32, b: *const i32, out: *mut i32) {
    let i = _workitem_id_x() as isize;
    *out.offset(i) = *a.offset(i) + *b.offset(i);
    //out[i] = a[i] + b[i];
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Found these in llvm-project/llvm/include/llvm/IR/IntrinsicsAMDGPU.td
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.amdgcn.s.barrier"]
    fn syncthreads() -> ();
    #[link_name = "llvm.amdgcn.wave.barrier"]
    fn syncwaves() -> ();

    #[link_name = "llvm.amdgcn.workgroup.id.x"]
    fn workgroup_id_x() -> i32;
    #[link_name = "llvm.amdgcn.workgroup.id.y"]
    fn workgroup_id_y() -> i32;
    #[link_name = "llvm.amdgcn.workgroup.id.z"]
    fn workgroup_id_z() -> i32;

    #[link_name = "llvm.amdgcn.workitem.id.x"]
    fn workitem_id_x() -> i32;
    #[link_name = "llvm.amdgcn.workitem.id.y"]
    fn workitem_id_y() -> i32;
    #[link_name = "llvm.amdgcn.workitem.id.z"]
    fn workitem_id_z() -> i32;
}

/// Synchronizes all threads in the block.
#[inline]
pub unsafe fn _syncthreads() -> () {
    syncthreads()
}

/// Synchronizes all waves.
#[inline]
pub unsafe fn _syncwaves() -> () {
    syncwaves()
}

/// x-th thread-block index.
#[inline]
pub unsafe fn _workgroup_id_x() -> i32 {
    workgroup_id_x()
}

/// y-th thread-block index.
#[inline]
pub unsafe fn _workgroup_id_y() -> i32 {
    workgroup_id_y()
}

/// z-th thread-block index.
#[inline]
pub unsafe fn _workgroup_id_z() -> i32 {
    workgroup_id_z()
}

/// x-th thread index.
#[inline]
pub unsafe fn _workitem_id_x() -> i32 {
    workitem_id_x()
}

/// y-th thread index.
#[inline]
pub unsafe fn _workitem_id_y() -> i32 {
    workitem_id_y()
}

/// z-th thread index.
#[inline]
pub unsafe fn _workitem_id_z() -> i32 {
    workitem_id_z()
}
