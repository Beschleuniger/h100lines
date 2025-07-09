use reqwest::*;

pub async fn validateOrReplaceLink(link: String, name: &String, artist: &String) -> String {

    // search for link validity, otherwise call searchLink() 

    print!("Validate: ");
    println!("{}", link);

    // let api = std::env::var("YOUTUBE").expect("YOUTUBE api key not avaiable, please provide it in the .env file");

    // let client: Client = reqwest::Client::new();

    // let res = client
    //                 .get(link)
    //                 .query(&[
    //                     ("key", api),
    //                     ("part", "snippet".to_owned()),
    //                     ("q", query),
    //                     ("maxResults", "5".to_owned()),
    //                     ("type", "video".to_owned())
    //                 ]).send()
    //                 .await?
    //                 .json::<YouTubeSearchResponse>()
    //                 .await?;

    




    return link;
    searchLink(name, artist);
}

pub fn searchLink(name: &String, artist: &String) -> String {

    println!("search");


    // search on youtube for song and get the link

    todo!()
}