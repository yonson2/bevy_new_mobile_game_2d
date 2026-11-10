//! Seeded randomness. Every gameplay draw comes from [`GameRng`], never from a
//! global RNG, so a run seed plays the same run on every target and at any frame rate.
//!
//! Each purpose draws from its own [`Stream`], seeded from the run seed and the stream's
//! key: a draw in one stream never moves another, so a draw in combat cannot change the
//! next shop. Add a stream for each new purpose.
//!
//! Draw `u64`, never `usize`: 32-bit targets draw `usize` differently.
//! `clippy.toml` forbids `fastrand::Rng::usize` and the methods built on it.

use std::cmp::Ordering;

use bevy::{ecs::schedule::SingleThreadedExecutor, prelude::*};

pub(super) fn plugin(app: &mut App) {
    let seed = random_seed();
    info!("run seed {seed}");
    app.insert_resource(GameRng::new(seed));
    // One thread: the gameplay systems run in one fixed order, so systems that draw from
    // the same stream draw in the same order on every run.
    app.edit_schedule(FixedUpdate, |schedule| {
        schedule.set_executor(SingleThreadedExecutor::new());
    });
}

/// What a draw is for. Each purpose has its own stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stream {
    /// Where and how fast the demo's enemies come in.
    Spawns,
}

impl Stream {
    const ALL: [Stream; 1] = [Stream::Spawns];

    /// Part of the stream's seed. A published seed (a daily run) plays another run if a
    /// key changes, so a key never changes.
    fn key(self) -> &'static str {
        match self {
            Stream::Spawns => "spawns",
        }
    }
}

/// The run's randomness: one RNG per [`Stream`].
#[derive(Resource)]
pub struct GameRng {
    streams: [fastrand::Rng; Stream::ALL.len()],
}

impl GameRng {
    pub fn new(seed: u64) -> Self {
        Self {
            streams: Stream::ALL.map(|s| fastrand::Rng::with_seed(stream_seed(seed, s.key()))),
        }
    }

    pub fn stream(&mut self, stream: Stream) -> &mut fastrand::Rng {
        &mut self.streams[stream as usize]
    }

    /// Each stream's state, for the replay check's hash. A run save can keep it too:
    /// `fastrand::Rng::with_seed` on each value gives the streams back.
    #[cfg(feature = "dev_tools")]
    pub fn state(&self) -> [u64; Stream::ALL.len()] {
        self.streams.each_ref().map(fastrand::Rng::get_seed)
    }
}

/// The seed of a new run: the only use of the global `fastrand`.
pub fn random_seed() -> u64 {
    fastrand::u64(..)
}

/// A stream's seed: FNV-1a over the run seed and the key, then SplitMix64's finalizer.
/// Integer math only, so every target gets the same value.
fn stream_seed(seed: u64, key: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in seed.to_le_bytes().into_iter().chain(key.bytes()) {
        h ^= u64::from(byte);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h = (h ^ (h >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h = (h ^ (h >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    h ^ (h >> 31)
}

/// The order a system goes over entities in when it draws for several of them, or picks
/// one: by place, x then y. Query order follows the archetypes, which an effect
/// or a skin changes, so a seed would play another way.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the demo draws for no entity; a system that does sorts with it"
    )
)]
pub fn by_place(a: Vec2, b: Vec2) -> Ordering {
    a.x.total_cmp(&b.x).then(a.y.total_cmp(&b.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draws(rng: &mut GameRng, stream: Stream) -> Vec<u64> {
        (0..8).map(|_| rng.stream(stream).u64(..)).collect()
    }

    #[test]
    fn same_seed_and_stream_give_the_same_draws() {
        for stream in Stream::ALL {
            let (mut a, mut b) = (GameRng::new(42), GameRng::new(42));
            assert_eq!(draws(&mut a, stream), draws(&mut b, stream));
        }
    }

    #[test]
    fn streams_and_seeds_differ() {
        assert_ne!(stream_seed(42, "spawns"), stream_seed(42, "drops"));
        assert_ne!(
            draws(&mut GameRng::new(42), Stream::Spawns),
            draws(&mut GameRng::new(43), Stream::Spawns)
        );
    }

    /// A published seed must keep its run: a change to `stream_seed`, a key or fastrand's
    /// algorithm fails this test.
    #[test]
    fn draws_are_pinned() {
        let mut rng = GameRng::new(1);
        assert_eq!(rng.stream(Stream::Spawns).u64(..), 10841015783620314663);
        assert_eq!(rng.stream(Stream::Spawns).u64(..100), 45);
    }

    #[test]
    fn by_place_sorts_by_x_then_y() {
        let mut places = [Vec2::new(1., 0.), Vec2::new(0., 2.), Vec2::new(0., -1.)];
        places.sort_by(|a, b| by_place(*a, *b));
        assert_eq!(
            places,
            [Vec2::new(0., -1.), Vec2::new(0., 2.), Vec2::new(1., 0.)]
        );
    }
}
