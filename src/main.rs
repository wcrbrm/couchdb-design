use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

mod designview;

#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "couchdb-design",
    about = "A command line interface to work with CouchDB design documents as YAML configurations"
)]
struct Opt {
    /// Local YAML file to be uploaded as design document. 
    /// If not provided, it will read URLs and display it 
    /// as YAML file in stdout
    #[structopt(short="f", long)]
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,

    /// URL of the couch design document to be read of updated
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let url = opt.url.clone();
    if let Some(_) = opt.file {
        // designview::upload_localfile(url, path_yaml).await?;
    } else {
        designview::display(url).await?;
    }
    Ok(())
}
