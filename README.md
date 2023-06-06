# xng-rs-log

This implements a logger that uses XalPutChar from the XAL.
See also [xng-rs](https://github.com/aeronautical-informatics/xng-rs).

```rs
#![no_std]

use xng_rs_log::XalLogger;

static LOGGER: XalLogger = XalLogger;

#[no_mangle]
pub extern "C" fn main() {
    unsafe { log::set_logger_racy(&LOGGER).unwrap() };
    log::set_max_level(log::LevelFilter::Info);
    // Run your code
}
```
