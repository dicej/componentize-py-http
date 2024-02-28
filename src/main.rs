use std::env;
use std::time::Instant;

use anyhow::Result;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};
use wasmtime_wasi::preview2::{self, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::proxy;
use wasmtime_wasi_http::{
    self,
    // bindings::http::types as http_types, body::HyperOutgoingBody, hyper_response_error,
    WasiHttpCtx,
    WasiHttpView,
};

wasmtime::component::bindgen!({
    world: "hello-http",
    path: "wit",
    async: true,
    with: {
        "wasi:cli/stderr": preview2::bindings::cli::stderr,
        "wasi:cli/stdin": preview2::bindings::cli::stdin,
        "wasi:cli/stdout": preview2::bindings::cli::stdout,
        "wasi:clocks/monotonic-clock": preview2::bindings::clocks::monotonic_clock,
        "wasi:clocks/timezone": preview2::bindings::clocks::timezone,
        "wasi:clocks/wall-clock": preview2::bindings::clocks::wall_clock,
        "wasi:http/incoming-handler": wasmtime_wasi_http::bindings::http::incoming_handler,
        "wasi:http/outgoing-handler": wasmtime_wasi_http::bindings::http::outgoing_handler,
        "wasi:http/types": wasmtime_wasi_http::bindings::http::types,
        "wasi:io/streams": preview2::bindings::io::streams,
        "wasi:io/poll": preview2::bindings::io::poll,
        "wasi:random/random": preview2::bindings::random::random,
    },
});

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let world = &args[1];
    let filename = &args[2];

    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdio();
    let table = ResourceTable::new();
    let wasi = builder.build();

    let mut config = Config::new();
    config.cache_config_load_default().unwrap();
    config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    struct CommandCtx {
        table: ResourceTable,
        wasi: WasiCtx,
        http: WasiHttpCtx,
    }
    impl WasiView for CommandCtx {
        fn table(&self) -> &ResourceTable {
            &self.table
        }
        fn table_mut(&mut self) -> &mut ResourceTable {
            &mut self.table
        }
        fn ctx(&self) -> &WasiCtx {
            &self.wasi
        }
        fn ctx_mut(&mut self) -> &mut WasiCtx {
            &mut self.wasi
        }
    }

    impl WasiHttpView for CommandCtx {
        fn table(&mut self) -> &mut ResourceTable {
            &mut self.table
        }
        fn ctx(&mut self) -> &mut WasiHttpCtx {
            &mut self.http
        }
    }

    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);
    preview2::command::add_to_linker(&mut linker)?;
    proxy::add_only_http_to_linker(&mut linker)?;

    let host = CommandCtx {
        table: table,
        wasi: wasi,
        http: WasiHttpCtx {},
    };
    let mut store = Store::new(&engine, host);

    let start = Instant::now();
    let component = Component::from_file(&engine, filename).unwrap();

    let (instance, _instance) =
        HelloHttp::instantiate_async(&mut store, &component, &linker).await?;

    let input = "Hello HTTP".to_string();
    let res = instance.call_hello(&mut store, &input).await?;
    println!("{:?}", res);

    let duration = start.elapsed();
    println!("Time elapsed in {:?} is: {:?}", world, duration);

    Ok(())
}
