#![no_std]
#![no_main]

use aya_bpf::{
    macros::sock_addr,
    programs::SockAddrContext,
};
use aya_log_ebpf::info;

#[sock_addr(connect4)]
pub fn cgroup_sock_test(ctx: SockAddrContext) -> i32 {
    match unsafe { try_cgroup_sock_test(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_cgroup_sock_test(ctx: SockAddrContext) -> Result<i32, i32> {
    let family = (*ctx.sock_addr).user_family;
    let addr = (*ctx.sock_addr).user_ip4;
    info!(&ctx, "CONNECT family: {} ip: {}", family, addr);
    Ok(1)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
