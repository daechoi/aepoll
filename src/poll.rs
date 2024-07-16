#![allow(dead_code)]

use crate::ffi;
use std::io::{self, Result};

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let res = unsafe { ffi::epoll_create(1) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            registry: Registry { raw_fd: res },
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = unsafe { ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        };

        unsafe {
            events.set_len(res as usize);
        };
        Ok(())
    }
}

pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    pub fn register(&self, fd: i32, token: i32, interests: i32) -> Result<()> {
        todo!("implement Registry::register")
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!("implement Drop for Registry")
    }
}
