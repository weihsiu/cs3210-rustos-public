// FIXME: Make me pass! Diff budget: 25 lines.

// I AM NOT DONE

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16),
}

// What traits does `Duration` need to implement?
impl From<&Duration> for u64 {
    fn from(dur: &Duration) -> Self {
        match *dur {
            Duration::MilliSeconds(n) => n,
            Duration::Seconds(n) => n as u64 * 1000,
            Duration::Minutes(n) => n as u64 * 60
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        let x: u64 = self;
        let y: u64 = other;
        x == y
    }
}

#[test]
fn traits() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
