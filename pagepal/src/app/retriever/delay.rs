use chrono::Duration;
use tokio::time::{interval_at, sleep_until, Instant, Interval};

#[derive(Debug)]
pub struct Delay(pub Instant, pub Interval);
impl Delay {
    pub async fn delay_if(&mut self, delay: Duration) -> &mut Self {
        let until = self.0 + delay.to_std().unwrap();
        if until > Instant::now() {
            sleep_until(until).await;
            self.1.tick().await;
        }
        self.0 = Instant::now();
        self
    }
}
impl Default for Delay {
    fn default() -> Self {
        let dur = Duration::milliseconds(100).to_std().unwrap();
        Self(Instant::now(), interval_at(Instant::now() + dur, dur))
    }
}
