use timex_core::{git::{
    diff::{get_commit_diff, CommitTreeIterator, Walker},
    error::GitError,
}, project::workload::Workload};




#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    dotenv::dotenv().ok();
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let workload = Workload::from_toml_file("workload.toml")?;
    let report = workload.generate_report(&openai_api_key).await?;
    println!("{:?}", report);
    Ok(())
}
