mod cli;

use std::env;
use std::path::PathBuf;

use qcos::client::Client;
use qcos::objects::{mime, ErrNo};
use chrono::Local;
use walkdir::WalkDir;
use anyhow::{Result, anyhow};

pub use cli::Cli;
use cli::*;

pub async fn upload(client: &Client, opts: &UpOpts) -> Result<()> {
    let filename = get_bkname(opts)?;

    let remote = format!("{}/{}", opts.remote_dir, filename);
    let file_path = PathBuf::from(&opts.local_dir).join(filename);

    let content_type = mime_guess::MimeGuess::from_path(&file_path)
        .first_or(mime::APPLICATION_OCTET_STREAM);

    let res = client
        .clone()
        .put_big_object_progress_bar(
            &file_path,
            &remote,
            Some(content_type),     //自动获取文件类型
            Some(qcos::objects::StorageClassEnum::StandardIa),    //低频存储,默认标准STANDARD
            None,
            Some(opts.part_size),    
            Some(5),        //5个线程
            None,
        )
        .await;

    if res.error_no == ErrNo::SUCCESS {
        println!("success");
    } else {
        println!("[{}]: {}", res.error_no, res.error_message);
    }

    Ok(())
}

pub async fn download(client: &Client, opts: &DownOpts) -> Result<()> {
    let auth_key = env::var("AUTH_KEY")
	.map_err(|_| anyhow!("AUTH_KEY missing"))?;

    if auth_key  != "test1234" {
        return Err(anyhow!("AUTH_KEY wrong"));
    }

    let filename = opts.filename.split("/").last().unwrap_or(opts.filename.as_str());
    let file_path = PathBuf::from(&opts.local_dir).join(filename);
    
    let res = client
        .get_object_progress_bar(
            &opts.filename,
    	&file_path.to_string_lossy().into_owned(),
            Some(5),
            None
        )
        .await;
    
    if res.error_no == ErrNo::SUCCESS {
        println!("success");
    } else {
        println!("[{}]: {}", res.error_no, res.error_message);
    }

    Ok(())
}

fn get_bkname(opts: &UpOpts) -> Result<String> {
    let dep = match opts.date_type {
        1 => "-",
        2 => "_",
        _ => "",
    };
    let fmt = format!("%Y{0}%m{0}%d", dep);
    let date_str = Local::now().format(&fmt).to_string();

    let entries = WalkDir::new(&opts.local_dir)
        .into_iter()
        .filter_map(|e| e.ok());

    entries
        .filter(|e| e.file_type().is_file())
        .find_map(|e| {
            let name = e.file_name().to_string_lossy();
            name.contains(&date_str)
                .then(|| name.into_owned())
        })
        .ok_or(anyhow!("bk file not found"))
}

// -------------------------------------------------------
// entrypoint
pub async fn run(args: Cli) -> Result<()> {
    dotenvy::dotenv()
	.map_err(|e| anyhow!("Failed to load .env file: {e}"))?;

    let client = Client::new(
        env::var("SECRET_ID")?,
        env::var("SECRET_KEY")?,
        env::var("BUCKET_NAME")?,
        env::var("BUCKET_REGION")?,
    );

    match &args.cmd {
        Commands::Upload(opts) => upload(&client, opts).await?,
        Commands::Download(opts) => download(&client, opts).await?,
    }

    Ok(())
}
