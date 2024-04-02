use examples::{DemoService, ReqDto, ResDto};
use fusen::fusen_common::url::UrlConfig;
use fusen::{
    fusen_common::{
        self,
        server::{Protocol, RpcServer},
        FusenResult,
    },
    fusen_macro::{self, asset},
    register::nacos::NacosConfig,
    server::FusenServer,
};
use fusen_macro::fusen_server;
use tracing::info;

#[derive(Clone, Debug)]
struct DemoServiceImpl {
    _db: String,
}

#[fusen_server(package = "org.apache.dubbo.springboot.demo", version = "1.0.0")]
#[asset(id = "DemoService" ,method = GET)]
impl DemoService for DemoServiceImpl {
    #[asset(path="/sayHello2",method = POST)]
    async fn sayHello(&self, req: String) -> FusenResult<String> {
        info!("res : {:?}", req);
        return Ok("Hello ".to_owned() + &req);
    }
    async fn sayHelloV2(&self, req: ReqDto) -> FusenResult<ResDto> {
        info!("res : {:?}", req);
        return Ok(ResDto {
            str: "Hello ".to_owned() + &req.str + " V2",
        });
    }
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    fusen_common::logs::init_log();
    let server = DemoServiceImpl {
        _db: "我是一个DB数据库".to_string(),
    };
    let ds = server.get_info();
    println!("{:?}", ds);
    FusenServer::build()
        .add_register_builder(
            NacosConfig::builder()
                .server_addr("127.0.0.1:8848".to_owned())
                .app_name(Some("fusen-rust-server".to_owned()))
                .build()
                .boxed(),
        )
        .add_protocol(Protocol::HTTP2("8081".to_owned()))
        .add_fusen_server(Box::new(server))
        .run()
        .await;
}
