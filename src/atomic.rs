//ngx/os/unix/ngx_atomic.h

extern crate core;
use self::core::intrinsics::{
	atomic_cxchg_relaxed, atomic_xadd, atomic_load_relaxed,
	atomic_store_relaxed, atomic_fence_acqrel
};
use self::core::cell::UnsafeCell;
use self::core::marker::Sync;
use std::thread;
use std::time::Duration;

pub const NGX_HAVE_ATOMIC_OPS : usize = 1;
pub type NgxAtomicInt = i64;
pub type NgxAtomicUint = u64;

pub struct NgxAtomic {
	v: UnsafeCell<NgxAtomicUint>
}

unsafe impl Sync for NgxAtomic {} 

impl<'a> NgxAtomic {
	fn new(v: NgxAtomicUint) -> Self {
		NgxAtomic { v: UnsafeCell::new(v) }
	}
	
	fn cmp_set(&self, old: NgxAtomicUint, new: NgxAtomicUint) 
	 -> NgxAtomicUint {
		unsafe { ngx_atomic_cmp_set(self.v.get(), old, new) }
	}

	fn load(&self) -> NgxAtomicUint {
		unsafe { ngx_atomic_load(self.v.get()) }
	}

	fn store(&self, val: NgxAtomicUint) {
		unsafe { ngx_atomic_store(self.v.get(), val); }
	}
} 

#[cfg(NGX_PTR_SIZE = "8")]
const NGX_ATOMIC_T_LEN : usize = 19;
#[cfg(not(NGX_PTR_SIZE = "8"))]
const NGX_ATOMIC_T_LEN : usize = 10;

#[inline(always)]
unsafe fn ngx_atomic_cmp_set<T>(lock: *mut T, old: T, new: T) -> T {
	atomic_cxchg_relaxed(lock, old, new)
}	

#[inline(always)]
unsafe fn ngx_atomic_fetch_add<T>(value: *mut T, add: T) -> T {
	atomic_xadd(value, add)
}

#[inline(always)]
unsafe fn ngx_atomic_load<T>(src: *const T) -> T {
	atomic_load_relaxed(src)
}	

#[inline(always)]
unsafe fn ngx_atomic_store<T>(dst: *mut T, val: T) {
	atomic_store_relaxed(dst, val);
}

#[inline(always)]
pub unsafe fn ngx_memory_barrier() { atomic_fence_acqrel(); }

#[inline(always)]
pub fn ngx_cpu_pause() { thread::sleep(Duration::from_millis(0)); }

static ngx_cpu: usize = 2; //temporary static variable

pub unsafe fn ngx_spinlock(lock: &NgxAtomic, value: NgxAtomicUint, spin: usize) {
	while true {
		if lock.load() == 0 && 
				lock.cmp_set(0, value) == 0 { return; }

		if ngx_cpu > 1 {
			let mut n = 1;
			while n < spin {
				for _ in 0..n { ngx_cpu_pause(); }
				if lock.load() == 0 && 
				 		lock.cmp_set(0, value) == 0 { return; } 
				n = n << 1;		
			}
		}

		//ngx_sched_yield();
		thread::yield_now();
	}
}  

macro_rules! ngx_trylock {
	($lock:ident) => {
		if $lock.load() == 0 { $lock.cmp_set(0, 1); } 
	}
}

macro_rules! ngx_unlock {
	($lock:ident) => { $lock.store(0); }
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;
	use std::thread;
	use super::*;

	#[test]
	fn atomic_test() {
		let lock = Arc::new(NgxAtomic::new(0));

		let lock_clone = lock.clone();
		thread::spawn(move || 
			unsafe { 
				ngx_memory_barrier();
				lock_clone.cmp_set(0, 1);
				assert!(lock_clone.load() == 1);	
			}
		);

		let lock_clone2 = lock.clone();
		thread::spawn(move || 
			unsafe { lock_clone2.cmp_set(1, 0); }
		);   

		ngx_cpu_pause();
	}

	#[test]
	fn atomic_macro_test() {
		let lock = NgxAtomic::new(0);
		ngx_trylock!(lock);
		assert!(lock.load() == 1);
		ngx_unlock!(lock);
		assert!(lock.load() == 0);
	}

	#[test]
	fn spinlock_test() {
		let lock = Arc::new(NgxAtomic::new(1));
		
		let lock0 = lock.clone();
		thread::spawn(move ||
			unsafe {
				ngx_memory_barrier();
				lock0.cmp_set(1, 0);
			}
		);
		
		let lock1 = lock.clone();
		unsafe { 
			ngx_spinlock(lock1.as_ref(), 1, 1024);
		}

		assert!(lock.load() == 1); 
	}
}
