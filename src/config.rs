// core/ngx_config.h
extern crate core;
use self::core::intrinsics::transmute;

type NgxInt = i32; 
type NgxUint = u32; 
type NgxFlag = i32; 

const NGX_INT32_LEN : usize = 10; // sizeof("-2147483648") - 1;
const NGX_INT64_LEN : usize = 19; // sizeof("-9223372036854775808") - 1
const NGX_ALIGNMENT : usize = 8;  // sizeof(usigned long) 

macro_rules! ngx_align {
	($d:expr, $a:expr) => { ($d + ($a - 1)) & !($a - 1) }
}

//#define ngx_align_ptr(p, a)                                                   \
//   (u_char *) (((uintptr_t) (p) + ((uintptr_t) a - 1)) & ~((uintptr_t) a - 1))
macro_rules! ngx_align_ptr {
	($p:expr, $a:expr) => {
		{	
			let array = unsafe {
									  transmute::<u32, [u8; 4]>(
								 			(($p as u32) + (($a - 1) as u32)) & !(($a - 1) as u32))
							 		};
			std::str::from_utf8(&array.clone()).unwrap()
		}
	}
}

const NGX_INVALID_ARRAY_INDEX : u32 = 0x80000000; 

#[cfg(MAXHOSTNAMELEN)]
const NGX_MAXHOSTNAMELEN : u32 = MAXHOSTNAMELEN;
#[cfg(not(MAXHOSTNAMELEN))]
const NGX_MAXHOSTNAMELEN : u32 = 256;
