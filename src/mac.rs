use std::fmt;
use std::mem::MaybeUninit;
use std::ptr::addr_of_mut;
use std::option::Option;

use libc::socket;

const IFNAMSIZ: usize = 16;

#[derive(Debug, Default)]
pub struct MacAddress(pub [u8; 6]);

#[repr(C)]
#[derive(Debug)]
struct Ifreq {
    ifr_name: [libc::c_char; IFNAMSIZ],
    ifr_hwaddr: libc::sockaddr
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "-{:x}", b)?;
            } else {
                write!(f, "{:x}", b)?;
            }
        }

        Ok(())
    }
}

struct FdGuard(libc::c_int);

impl Drop for FdGuard {
    fn drop(&mut self) {
        let fd = self.0;
        if fd > 0 {
            unsafe {
                libc::close(fd);
            }
        }
    }
}

pub fn get_hardware_address<S: AsRef<str>>(iface: S) -> Option<MacAddress> {
    let fd = FdGuard(unsafe { socket(libc::AF_INET, libc::SOCK_STREAM, 0) });
    if fd.0 < 0 {
        return None;
    }

    let ifreq = {
        let mut ifreq = MaybeUninit::<Ifreq>::uninit();
        let ptr = ifreq.as_mut_ptr();

        let c_str = std::ffi::CString::new(iface.as_ref()).ok()?;

        unsafe {
            let mut ifr_name = [0 as libc::c_char; IFNAMSIZ];
            ifr_name.copy_from_slice(
              std::slice::from_raw_parts(
                  c_str.as_bytes_with_nul().as_ptr() as *mut u8 as *mut i8,
                  IFNAMSIZ
              )
          );

            addr_of_mut!((*ptr).ifr_name).write(ifr_name);
        };


        unsafe { ifreq.assume_init() }
    };


    let ret = unsafe { libc::ioctl(fd.0, libc::SIOCGIFHWADDR, &ifreq as *const Ifreq) };
    if ret < 0 {
        return None;
    }

    let mut addr = MacAddress::default();
    let addr_len = addr.0.len();
    addr.0.copy_from_slice(unsafe {
        std::slice::from_raw_parts(
            ifreq.ifr_hwaddr.sa_data.as_ptr() as *const u8,
            addr_len
        )}
    );

    Some(addr)
}
