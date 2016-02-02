// core/ngx_core.h

/* 
typedef struct ngx_module_s      ngx_module_t;
typedef struct ngx_conf_s        ngx_conf_t;
typedef struct ngx_cycle_s       ngx_cycle_t;
typedef struct ngx_pool_s        ngx_pool_t;
typedef struct ngx_chain_s       ngx_chain_t;
typedef struct ngx_log_s         ngx_log_t;
typedef struct ngx_open_file_s   ngx_open_file_t;
typedef struct ngx_command_s     ngx_command_t;
typedef struct ngx_file_s        ngx_file_t;
typedef struct ngx_event_s       ngx_event_t;
typedef struct ngx_event_aio_s   ngx_event_aio_t;
typedef struct ngx_connection_s  ngx_connection_t;
*/

//#if (NGX_THREADS)
//typedef struct ngx_thread_task_s  ngx_thread_task_t;
//#endif

//typedef void (*ngx_event_handler_pt)(ngx_event_t *ev);
//typedef void (*ngx_connection_handler_pt)(ngx_connection_t *c);

const NGX_OK : i32 = 0;
const NGX_ERROR : i32 = -1;
const NGX_AGAIN : i32 = -2;
const NGX_BUSY : i32 = -3;
const NGX_DONE : i32 = -4;
const NGX_DECLINED : i32 = -5;
const NGX_ABORT : i32 = -6;

const LF : char = '\n';
const CR : char = '\r';
const CRLF : &'static str = "\r\n";

//#define ngx_abs(value)       (((value) >= 0) ? (value) : - (value))
//#define ngx_max(val1, val2)  ((val1 < val2) ? (val2) : (val1))
//#define ngx_min(val1, val2)  ((val1 > val2) ? (val2) : (val1))

//void ngx_cpuinfo(void);

/*
#if (NGX_HAVE_OPENAT)
#define NGX_DISABLE_SYMLINKS_ON         1
#define NGX_DISABLE_SYMLINKS_NOTOWNER   2
#endif
*/

#[cfg(NGX_HAVE_OPENAT)]
enum DisableSymlinks { 
	NGX_DISABLE_SYMLINKS_ON,
	NGX_DISABLE_SYMLINKS_OFF,
	NGX_DISABLE_SYMLINKS_NOTOWNER,
}
