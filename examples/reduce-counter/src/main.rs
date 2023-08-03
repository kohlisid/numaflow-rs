use numaflow::function::start_uds_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reduce_handler = counter::Counter::new();

    start_uds_server(reduce_handler).await?;

    Ok(())
}

mod counter {
    use numaflow::function::{Datum, Message};
    use numaflow::function::{FnHandler, Metadata};
    use tokio::sync::mpsc::Receiver;
    use tonic::async_trait;

    pub(crate) struct Counter {}

    impl Counter {
        pub(crate) fn new() -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl FnHandler for Counter {
        async fn map_handle<T: Datum + Send + Sync + 'static>(&self, _: T) -> Vec<Message> {
            todo!()
        }

        async fn reduce_handle<
            T: Datum + Send + Sync + 'static,
            U: Metadata + Send + Sync + 'static,
        >(
            &self,
            keys: Vec<String>,
            mut input: Receiver<T>,
            md: &U,
        ) -> Vec<Message> {
            println!(
                "Entering into UDF {:?} {:?}",
                md.start_time(),
                md.end_time()
            );

            let mut counter = 0;
            // the loop exits when input is closed which will happen only on close of book.
            while let Some(_) = input.recv().await {
                counter += 1;
            }

            println!(
                "Returning from UDF {:?} {:?}",
                md.start_time(),
                md.end_time()
            );
            vec![Message {
                keys: keys.clone(),
                value: counter.to_string().into_bytes(),
                tags: vec![],
            }]
        }
    }
}
