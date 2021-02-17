pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

impl From<f64> for Duration {
    fn from(s: f64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn orbital_duration() -> Duration;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::orbital_duration().seconds
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_duration() -> Duration {
        Duration::from(7600543.81992)
    }
}

impl Planet for Venus {
    fn orbital_duration() -> Duration {
        Duration::from(19414149.052176)
    }
}

impl Planet for Earth {
    fn orbital_duration() -> Duration {
        Duration::from(31_557_600)
    }
}

impl Planet for Mars {
    fn orbital_duration() -> Duration {
        Duration::from(59354032.69008)
    }
}

impl Planet for Jupiter {
    fn orbital_duration() -> Duration {
        Duration::from(374355659.124)
    }
}

impl Planet for Saturn {
    fn orbital_duration() -> Duration {
        Duration::from(929292362.8848)
    }
}

impl Planet for Uranus {
    fn orbital_duration() -> Duration {
        Duration::from(2651370019.3296)
    }
}

impl Planet for Neptune {
    fn orbital_duration() -> Duration {
        Duration::from(5200418560.032)
    }
}
