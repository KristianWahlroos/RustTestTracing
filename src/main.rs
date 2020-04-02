use tracing::{event, info, span, Level};
use tracing_subscriber::FmtSubscriber;

fn hello_world() {
    info!("hello world");
}

fn event_span() {
    event!(Level::INFO, "something happened");
    let span = span!(Level::INFO, "my_span");
    {
        let _guard = span.enter();
        let span = span!(Level::DEBUG, "my_span1");
        {
            let _guard = span.enter();
            event!(Level::DEBUG, "something happened inside my_span");
            event!(Level::INFO, "something happened inside my_span");
        }
        event!(Level::INFO, "something happened inside my_span");
        event!(Level::INFO, "something happened inside my_span");
    }
    event!(Level::INFO, "something happened outside of my_span");
}

fn main() {
    FmtSubscriber::builder().with_max_level(Level::TRACE).init();
    hello_world();
    event_span();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        let my_subscriber = FmtSubscriber::new();
        tracing::subscriber::set_global_default(my_subscriber)
            .expect("settin tracing default failed");
        hello_world();
        event_span();
    }
}
