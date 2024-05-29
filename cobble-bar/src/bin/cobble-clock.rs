use std::{io, ptr, time::Duration};

use cobble_bar::sink::PrintSink;
use futures::{Sink, SinkExt as _};
use glib::timeout_future;
use libc::{timespec, CLOCK_REALTIME};

/// System time, according to the current timezone and system clock.
#[derive(Clone, Copy, PartialEq, Eq)]
struct LocalTime {
    hours: u8,
    minutes: u8,
    seconds: u8,
    nanoseconds: u64,
}

impl TryFrom<timespec> for LocalTime {
    type Error = anyhow::Error;

    fn try_from(value: timespec) -> Result<Self, Self::Error> {
        let mut local_time = libc::tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: ptr::null(),
        };

        let result = unsafe {
            libc::localtime_r(
                &value.tv_sec as *const i64,
                &mut local_time as *mut libc::tm,
            )
        };

        if result != &mut local_time as *mut libc::tm {
            return Err(io::Error::last_os_error().into());
        }

        Ok(LocalTime {
            hours: local_time.tm_hour.try_into()?,
            minutes: local_time.tm_min.try_into()?,
            seconds: local_time.tm_sec.try_into()?,
            nanoseconds: value.tv_nsec.try_into()?,
        })
    }
}

/// Get the current time from the system.
fn get_time() -> Result<LocalTime, anyhow::Error> {
    let mut timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    let result = unsafe { libc::clock_gettime(CLOCK_REALTIME, &mut timespec as *mut timespec) };
    if result != 0 {
        return Err(io::Error::last_os_error().into());
    }

    timespec.try_into()
}

pub struct Clock;
impl Clock {
    async fn run<S>(&self, mut sink: S) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Sink<String> + Unpin,
        <S as Sink<String>>::Error: std::error::Error + Send + Sync + 'static,
    {
        loop {
            let current_time = get_time()?;
            let LocalTime {
                hours,
                minutes,
                seconds,
                nanoseconds,
            } = current_time;
            sink.send(format!("{hours:0>2}:{minutes:0>2}\n")).await?;

            const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
            const NANOSECONDS_PER_MINUTE: u64 = 60 * NANOSECONDS_PER_SECOND;
            let wait_duration = Duration::from_nanos(
                NANOSECONDS_PER_MINUTE
                    - (u64::from(seconds) * NANOSECONDS_PER_SECOND)
                    - nanoseconds,
            );
            timeout_future(wait_duration).await;
        }
    }
}

fn main() {
    let clock = Clock;

    let context = glib::MainContext::default();
    context.block_on(clock.run(PrintSink)).unwrap();
}
