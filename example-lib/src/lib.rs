#![feature(custom_test_frameworks)]
#![test_runner(framework::runner)]

#[cfg(test)]
use framework;
use format::my_test;

#[cfg(test)]
use format::SimpleFnTest;


#[my_test]
fn foo() {}