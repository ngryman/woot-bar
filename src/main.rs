mod cpu_usage;

use async_macros::try_join;
use async_std::task;
use heim::{
  memory, units::{information, ratio}, Result
};
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<()> {
  let mem_fut = get_mem();
  let cpu_fut = get_cpu();
  let (used, cpu) = try_join!(mem_fut, cpu_fut).await?;

  print!("{:2.1}G {:2.0}%", used, cpu);

  Ok(())
}

async fn get_mem() -> Result<f64> {
  let mem = memory::memory().await?;
  let available = mem.available().get::<information::mebibyte>() as f64 / 1024.0;
  let total = mem.total().get::<information::mebibyte>() as f64 / 1024.0;
  let used = total - available;

  Ok(used)
}

async fn get_cpu() -> Result<f32> {
  let usage_1 = cpu_usage::usage().await?;
  task::sleep(Duration::from_secs(4)).await;
  let usage_2 = cpu_usage::usage().await?;
  let cpu = (usage_2 - usage_1).get::<ratio::percent>();

  Ok(cpu)
}
