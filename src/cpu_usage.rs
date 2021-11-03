use async_macros::try_join;
use heim::{
  cpu::{self, CpuTime}, units::{ratio, time, Ratio}, Result
};
use std::time::Instant;

///
/// Copy from PR: https://github.com/heim-rs/heim/issues/294
/// Waiting for next release to remove and use `heim::cpu::usage` instead

pub(crate) struct CpuUsage {
  cpu_count: u64,
  cpu_time: CpuTime,
  at: Instant,
}

impl std::ops::Sub<CpuUsage> for CpuUsage {
  type Output = Ratio;

  #[allow(clippy::suspicious_arithmetic_impl, clippy::cast_lossless)]
  fn sub(self, rhs: CpuUsage) -> Self::Output {
    let delta_proc = (self.cpu_time.user() - rhs.cpu_time.user())
      + (self.cpu_time.system() - rhs.cpu_time.system());
    let delta_time = self.at - rhs.at;

    // TODO: Can be replaced with a `delta_time.as_secs_f64()`
    // as soon as https://github.com/rust-lang/rust/issues/54361 will be stable
    const NANOS_PER_SEC: u32 = 1_000_000_000;
    let mut delta_time_secs =
      (delta_time.as_secs() as f64) + (delta_time.as_nanos() as f64) / (NANOS_PER_SEC as f64);

    // Time should calculated across all the cores available
    delta_time_secs *= self.cpu_count as f64;

    if delta_time_secs != 0.0 {
      let overall_cpus_ratio = delta_proc.get::<time::second>() / delta_time_secs;
      let single_cpu_ratio = overall_cpus_ratio * self.cpu_count as f64;

      Ratio::new::<ratio::ratio>(single_cpu_ratio as f32)
    } else {
      Ratio::new::<ratio::ratio>(0.0)
    }
  }
}

pub(crate) async fn usage() -> Result<CpuUsage> {
  let time_fut = cpu::time();
  let count_fut = cpu::logical_count();
  let (cpu_time, cpu_count) = try_join!(time_fut, count_fut).await?;

  Ok(CpuUsage {
    cpu_count,
    cpu_time,
    at: Instant::now(),
  })
}
