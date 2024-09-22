use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use thread::DatagenThread;

mod rand;
mod thread;

#[derive(Default)]
struct AtomicStats {
    games: AtomicUsize,
    positions: AtomicUsize,
}

impl AtomicStats {
    pub fn update(&self, positions: usize) {
        self.positions.fetch_add(positions, Ordering::Relaxed);
        self.games.fetch_add(1, Ordering::Relaxed);
    }

    pub fn games(&self) -> usize {
        self.games.load(Ordering::Relaxed)
    }

    pub fn positions(&self) -> usize {
        self.positions.load(Ordering::Relaxed)
    }
}

pub fn run<G: mentor::Game>() {
    let threads = parse_args();

    assert!(threads > 0, "Number of threads must be at least 1.");

    let params = mentor::helper::MctsParameter::default();
    let settings = mentor::helper::SearchSettings {
        max_time: Some(1000),
        max_nodes: usize::MAX,
    };

    let abort_base = AtomicBool::new(false);
    let stats_base = AtomicStats::default();

    std::thread::scope(|s| {
        let abort = &abort_base;
        let stats = &stats_base;

        for thread in 0..threads {
            s.spawn(move || {
                let mut datagen = DatagenThread::<G>::new(params, settings);
                datagen.run(abort, stats);

                println!(
                    "info thread {} games {} positions {}",
                    thread,
                    datagen.games(),
                    datagen.size()
                );
            });
        }

        games::handle_input(abort, |command, _| match command {
            "info" => println!(
                "info games {} positions {}",
                stats.games(),
                stats.positions()
            ),
            "stop" => abort.store(true, Ordering::Relaxed),
            _ => {}
        });
    });
}

fn parse_args() -> usize {
    let args: Vec<String> = std::env::args().collect();
    let mut args = args.iter().skip(1);

    let mut threads: Option<usize> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--threads" => {
                threads = Some(
                    args.next()
                        .expect("Argument for number of threads.")
                        .parse()
                        .expect("Number of threads."),
                )
            }
            _ => panic!("Unknown argument {:?}.", arg),
        }
    }

    threads.expect("Argument for threads.")
}
