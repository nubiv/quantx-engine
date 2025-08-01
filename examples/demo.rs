use quantx_engine::trace::init_json_tracing;
use tracing::info;

fn main() {
    init_json_tracing();

    info!("Starting QuantX Engine Demo");
}
