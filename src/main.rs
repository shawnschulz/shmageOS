#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(shos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use shos::panic;
use shos::kernel_main;
use shos::eh_personality;
use shos::abort;
