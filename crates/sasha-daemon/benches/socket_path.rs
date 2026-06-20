use criterion::{criterion_group, criterion_main, Criterion};
use std::{env::var, hint::black_box, path::{Path, PathBuf}, sync::Arc};

struct ListenerWithPath {
    socket_path: PathBuf
}

impl ListenerWithPath {
    fn new(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }
}

#[derive(Clone)]
struct NiriConnector {
    socket_path: Arc<PathBuf>
}

impl NiriConnector {
    fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path: Arc::from(socket_path)
        }
    }
}

struct ListenerWithConnector {
    connector: NiriConnector
}

impl ListenerWithConnector {
    fn new(connector: NiriConnector) -> Self {
        Self { connector }
    }
}


fn bench_cloned_path_setup(c: &mut Criterion) {
    let socket_path: String = var("NIRI_SOCKET").expect("Cannot find NIRI_SOCKET var");
    let path: PathBuf = PathBuf::from(socket_path);

    c.bench_function("setup with cloned Path", |b| {
        b.iter(|| {
            let niri_listener = ListenerWithPath::new(black_box(path.clone()));
            let command_listener = ListenerWithPath::new(black_box(path.clone()));
            black_box((niri_listener, command_listener));
        })
    });
}

fn bench_arc_path_setup(c: &mut Criterion) {
    let socket_path: String = var("NIRI_SOCKET").expect("Cannot find NIRI_SOCKET var");
    let path: PathBuf = PathBuf::from(socket_path);

    let niri_connector: NiriConnector = NiriConnector::new(path);

    c.bench_function("setup with arc Path", |b| {
        b.iter(|| {
            let niri_listener = ListenerWithConnector::new(niri_connector.clone());
            let command_listener = ListenerWithConnector::new(niri_connector.clone());
            black_box((niri_listener, command_listener));
        })
    });
}

criterion_group!(
    benches,
    bench_cloned_path_setup,
    bench_arc_path_setup
);
criterion_main!(benches);


