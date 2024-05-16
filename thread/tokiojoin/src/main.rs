use tokio::time::sleep;
use std::time::Duration;



#[tokio::main]
async fn main(){

    let address = "https://demo.com/";
    let local_path = "/home/iyappan/Documents";

    let (net, lo) = tokio::join!(download(address.to_owned()), import_file(local_path.to_owned()));


    // tokio join is wait to complete the both tasks, to be completed.


    println!("Task Completed..");

    println!("Task 1  {}", net);

    println!("Task 2  {}", lo);



}

async fn download(url: String) -> String {

    println!("Started Downloading: {}", url);

    sleep(Duration::from_secs(3)).await;


    println!("Download Completed.");

    String::from("<html>Dummy Content</html>")

}

async fn import_file(path: String) -> String {

    println!("Importing file: {}", path);

    sleep(Duration::from_secs(6)).await;


    println!("Import Completed.");

    String::from("File content is Json")

}