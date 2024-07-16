#![allow(dead_code)]

pub const EPOLL_CTL_ADD: i32 = 1;
/// bitflag for read operations on the file handle
pub const EPOLLIN: i32 = 0x1;

/// bitflag set to edge triggered mode
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {

    /// https://man7.org/linux/man-pages/man2/epoll_create.2.html
    pub fn epoll_create(size: i32) -> i32;
    pub fn close(fd: i32) -> i32;

    /// https://man7.org/linux/man-pages/man2/epoll_ctl.2.html
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *const Event) -> i32;

    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub(crate) events: u32,

    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
