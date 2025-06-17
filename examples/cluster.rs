use hydro_deploy::Deployment;

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();

    let flow = hydro_lang::FlowBuilder::new();
    let leader = flow.process();
    let workers = flow.cluster();
    code::cluster::cluster(&leader, &workers);

    let _nodes = flow
        .with_process(&leader, deployment.Localhost())
        .with_cluster(&workers, vec![deployment.Localhost(); 8])
        .deploy(&mut deployment);

    deployment.run_ctrl_c().await.unwrap();
}
