use yew_agent::Registrable;
use yt_yew_021::oneshot::FibonacciTask;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    FibonacciTask::registrar().register();
}
