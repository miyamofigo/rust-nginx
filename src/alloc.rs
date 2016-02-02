// os/unix/ngx_alloc.h/c
extern crate core;
extern crate libc;
extern crate rlibc;

extern "C" {
	fn posix_memalign(memptr: *mut *mut libc::c_void,
                    align: libc::size_t,
                    size: libc::size_t) -> libc::c_int;
}

//TODO: A logger should be added these funcs below.

#[inline]
pub unsafe fn ngx_alloc(size: usize) 
 -> Result<*mut u8, &'static str> {
	let ptr = libc::malloc(size as libc::size_t) as *mut u8; 
	if ptr.is_null() {
		return Err("failed to allocate memory.");
	}
	Ok(ptr) 
}

#[inline(always)]
pub unsafe fn ngx_free(ptr: *mut u8) {
	libc::free(ptr as *mut libc::c_void);
}

#[inline]
pub unsafe fn ngx_calloc(size: usize) 
 -> Result<*mut u8, &'static str> {
	let result = ngx_alloc(size);
	if result.is_ok() {
		result.map(|ptr| { ngx_memzero(ptr, size); ptr })
	} else { result }	 
	
} 

//Should this be transfered or not?? 
#[inline(always)]
pub unsafe fn ngx_memzero(buf: *mut u8, size: usize) 
 -> *mut u8 {
	rlibc::memset(buf, 0, size); buf
}

#[inline]
pub unsafe fn ngx_memalign(align: usize, size: usize) -> *mut u8 {
	let mut out = core::ptr::null_mut();
	let err = posix_memalign(&mut out, align as libc::size_t,
													 size as libc::size_t);

	if err != 0 { core::ptr::null_mut() } 
	else { out as *mut u8 }
}

#[test]
fn allocate_test() {
	unsafe {
		let result = ngx_alloc(32);
		assert!(result.is_ok());
		ngx_free(result.unwrap()); 
	}

	unsafe { 
		let result = ngx_calloc(32);
		assert!(result.is_ok());
		let cptr = result.unwrap();
		for i in core::slice::from_raw_parts::<u8>(cptr, 32) {
			assert!(*i == 0);
		}
		ngx_free(cptr);
	} 

	//Should this be transfered or not?? 
	unsafe {
		let aptr = ngx_memalign(16, 312);
		ngx_free(aptr);
	} 
}
