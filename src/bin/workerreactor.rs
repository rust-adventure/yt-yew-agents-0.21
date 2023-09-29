use yew_agent::Registrable;
use yt_yew_021::reactor::TimeFormatReactor;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    TimeFormatReactor::registrar().register();
}
