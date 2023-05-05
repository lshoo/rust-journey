// Define a rpc server api
use crate::response::Response;
use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;

#[rpc(server, client)]
trait RpcService {
    #[method(name = "echo")]
    async fn echo(&self, msg: Vec<u8>) -> RpcResult<Response<String>>;

    #[method(name = "start")]
    async fn start(&self) -> RpcResult<()>;

    #[method(name = "publish")]
    async fn publish(&self, module_bytes: Vec<u8>) -> RpcResult<bool>;

    #[method(name = "execute_function")]
    async fn execute_function(&self, function_bytes: Vec<u8>) -> RpcResult<bool>;
}

#[derive(Debug, Default)]
pub struct RoochServer {
    counter: u64,
}

#[async_trait]
impl RpcServiceServer for RoochServer {
    async fn echo(&self, msg: Vec<u8>) -> RpcResult<Response<String>> {
        println!("before echo, the counter is {}", self.counter);
        // self.counter += 1;
        Ok(Response::ok(format!("{msg:?}: {}", self.counter)))
    }

    async fn start(&self) -> RpcResult<()> {
        println!("{:?} starting", self);
        Ok(())
    }

    async fn publish(&self, module_bytes: Vec<u8>) -> RpcResult<bool> {
        println!("{:?} pbulishing {:?}", self, module_bytes);
        Ok(true)
    }

    async fn execute_function(&self, function_bytes: Vec<u8>) -> RpcResult<bool> {
        println!("{:?} executing {:?}", self, function_bytes);
        Ok(true)
    }
}

trait IntoJsonRpcResult<T> {
    fn internal_call_error(self) -> RpcResult<T>;
}
