use futures::{sink::SinkExt, stream::StreamExt};
use std::time::Duration;
use time_humanize::HumanTime;
use yew_agent::reactor::{reactor, ReactorScope};

#[reactor(TimeFormatReactor)]
pub async fn time_format_reactor(
    mut scope: ReactorScope<u64, String>,
) {
    while let Some(input) = scope.next().await {
        let duration = Duration::from_secs(input);

        let output =
            format!("{}", HumanTime::from(duration));

        // sends output
        if scope.send(output).await.is_err() {
            // sender closed, the bridge is disconnected
            break;
        }
    }
}
