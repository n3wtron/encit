use std::sync::Arc;

use clap::{App, Arg, ArgMatches, SubCommand};
use futures::executor::block_on;

use crate::api::server::WebServer;
use crate::enc::EncItImpl;
use crate::{EncItConfig, EncItError};

pub fn web_cmd<'a>() -> App<'a, 'a> {
    SubCommand::with_name("web").arg(
        Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080"),
    )
}

pub fn web_exec<'a>(
    arg_matches: &'a ArgMatches<'a>,
    config: Arc<dyn EncItConfig>,
) -> Result<(), EncItError> {
    let port = arg_matches.value_of("port").unwrap();
    let host = format!("localhost:{}", port);
    println!("Starting server http://{}", &host);
    let enc_it = Arc::new(EncItImpl::new(config));
    let server = Box::leak(Box::new(WebServer::new(&host, enc_it)));
    block_on(server.start())
}
