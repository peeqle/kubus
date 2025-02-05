mod args;
mod executor;
mod kube;
mod smooth_operator;

use crate::args::*;
use crate::executor::execute;

static KUBERNETES_RUNNER: &str = "kubectl";

//todo prevent from using >1 operations
#[tokio::main]
async fn main() {
    kube::get_client().await;
    execute(parse_args()).await;
}
