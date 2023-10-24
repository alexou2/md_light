use headless_chrome::{Browser, LaunchOptionsBuilder};
use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

// the
pub struct NewEpisodeReleases {
    pub thumbnail: String,
    pub anime_name: String,
    pub last_episode_number: i32,
    pub link_to_last_episode: String,
    pub link_to_anime_page: String,
}

pub struct AnimeDescription {
    pub thumbnail: String,
    pub name: String,
    pub infos: Vec<String>,
    pub episodes_list: Vec<AnimeEpisode>,
}
pub struct AnimeEpisode {
    pub episode_name: String,
    pub link_to_episode: String,
}
// contains the iframe and other infos for the page to watch the episode
pub struct EpisodeInfo {
    pub iframe_link: String,
    pub current_episode_link: String,
    pub prev_episode_link: Option<String>,
    pub next_episode_link: Option<String>, //the next episode can ether have a null value or a string value
}
const BASE_URL: &str = "https://gogoanimehd.io";

async fn request_page(url: &str) -> Result<Html, reqwest::Error> {
    // let client = reqwest::blocking::get(url)?.text()?; //converting the request string to html
    let client = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&client);
    println!("request done!");
    Ok(document)
}

pub async fn get_updates() -> Result<Vec<NewEpisodeReleases>, Box<dyn Error>> {
    // the list that will contain all of the data about the anime in the app
    let mut data: Vec<NewEpisodeReleases> = Vec::new();

    //the regex that will be used to remove the part of the episode link inorder to get the link to the anime
    let episode_regex: Regex = Regex::new("-episode-[0-9]*$").unwrap();
    let filter_number_regex: Regex = Regex::new("[^0-9]*").unwrap();

    // doing a request to the website
    // let client = reqwest::blocking::get(BASE_URL)?.text()?; //converting the request string to html
    // let document = Html::parse_document(&client);
    let document = request_page(BASE_URL).await?;

    // all of the selectors used for html parsing
    let list_selector = Selector::parse("ul.items").unwrap(); //the <ul> containing all ig the new episode releases
    let list_element_selector = Selector::parse("li").unwrap(); // selector for the new anime
    let image_selector = Selector::parse("img").unwrap(); //selector for the anime thumbnail
    let link_selector = Selector::parse("a").unwrap(); // selector for the link to the anime
    let paragraph_selector = Selector::parse("p.episode").unwrap(); //

    let anime_list = document.select(&list_selector).next().unwrap();

    // a loop that processes each anime in the new episode release and adds the data in the data vector
    for element in anime_list.select(&list_element_selector) {
        // gets the link to the episode's page
        let link_to_episode = element
            .select(&link_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string();
        // link_to_episode = format!("{}{}", BASE_URL, link_to_episode);

        //removes the episode name from the url in order to get the link to the anime info page
        let anime_name_part = episode_regex.replace(&link_to_episode, "").to_string();
        // formats the episode link to include the base url in it instead of having the relative path
        let link_to_anime_page = format!("{}/category{}", BASE_URL, anime_name_part);

        // the thumbnail of the anime
        let thumbnail_href = element
            .select(&image_selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap_or("image not available")
            .to_string();

        // the name of the anime
        let anime_name = element
            .select(&link_selector)
            .next()
            .unwrap()
            .value()
            .attr("title")
            .unwrap_or("no anime found")
            .to_string();

        // the last episode aired for the anime
        let episode: Vec<&str> = element
            .select(&paragraph_selector)
            .next()
            .unwrap()
            .text()
            .collect();

        // transforming the episode name(string) into an i32
        let episode_number_string = filter_number_regex.replace(episode[episode.len() - 1], ""); //taking the last
        println!("{}", episode_number_string);
        let episode_number_int: i32 = episode_number_string.parse().unwrap();

        let new_episode_data = NewEpisodeReleases {
            thumbnail: thumbnail_href,
            anime_name: anime_name,
            last_episode_number: episode_number_int,
            link_to_last_episode: format!("{}{}", BASE_URL, link_to_episode),
            link_to_anime_page: link_to_anime_page,
        };
        data.push(new_episode_data);
    }
    Ok(data)
}

// doing a get request to the specified url and returns infos like the name, episodes, genre, etc.. to be displayed on the anime info app
pub fn get_anime_info(anime_link: String) -> Result<AnimeDescription, Box<dyn Error>> {
    // declaring every selector for parsing the html returned from the get request for the anime info page in the tauri app

    // the selectors used for the general anime info
    let anime_info_selector: Selector = Selector::parse("div.anime_info_body_bg").unwrap(); // the selector for where all of the info is(top of the page), including thumbnail, name and other infos
    let anime_name_selector: Selector = Selector::parse("p.type").unwrap(); //the
    let image_selector = Selector::parse("img").unwrap(); //selector for the anime thumbnail

    // the selectors used to get the links/info of the episodes
    let list_selector: Selector = Selector::parse("ul#episode_related").unwrap(); //the <ul> containing all of the anime's episodes
    let list_element_selector: Selector = Selector::parse("li").unwrap(); // selector for the episode
    let link_selector = Selector::parse("a").unwrap(); // selector for the link to the anime

    //doing a get request to the requested page
    // let client = reqwest::blocking::get(anime_link)
    //     .expect("error while doing get request to website")
    //     .text()
    //     .expect("error while doing get request to website");
    let client = scrape_with_browser(
        &anime_link,
        vec!["div.name", "div.anime_info_body_bg", "ul#episode_related"],
    )
    .unwrap();

    //converting the request string to html
    let document = Html::parse_document(&client);
    let anime_properties = document.select(&anime_info_selector).next().unwrap();

    let thumbnail = anime_properties
        .select(&image_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap_or("link not available")
        .to_string();

    let anime_name: Vec<_> = anime_properties
        .select(&anime_name_selector)
        .next()
        .unwrap()
        .text()
        .collect();
    let anime_name_string = anime_name.join(" ");

    let mut infos: Vec<String> = Vec::new();
    // for property in document.select(&anime_info_selector) {
    for i in document.select(&anime_name_selector) {
        let data: Vec<_> = i.text().collect();
        let infos_string = data.join(" ").replace("\n", "").replace("\t", "");

        println!("info : {}", infos_string);
        infos.push(infos_string);
    }
    let mut episodes: Vec<AnimeEpisode> = Vec::new();

    let episode_list = document.select(&list_selector).next().ok_or("error")?;

    for ep in episode_list.select(&list_element_selector) {
        let mut episode_link = ep
            .select(&link_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .trim()
            .to_string();
        let episode_name = ep.text().collect();
        // let episode_name = "ep".to_string();
        // println!("episode link:\n{}\n{}", episode_link, episode_name);
        episode_link = format!("{}{}", BASE_URL, episode_link);

        let episode_data: AnimeEpisode = AnimeEpisode {
            episode_name: episode_name,
            link_to_episode: episode_link,
        };
        // println!("push");
        episodes.push(episode_data)
    }

    let anime_info = AnimeDescription {
        thumbnail: thumbnail,
        name: anime_name_string,
        infos: infos,
        episodes_list: episodes,
    };

    Ok(anime_info)
}

pub async fn get_episode_data(episode_link: String) -> Result<EpisodeInfo, Box<dyn Error>> {
    let prev_episode_selector = Selector::parse("div.anime_video_body_episodes_l").unwrap();
    let next_episode_selector = Selector::parse("div.anime_video_body_episodes_r").unwrap();
    let iframe_selector = Selector::parse("iframe").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    // let client = scrape_with_browser(
    //     &episode_link,
    //     vec![
    //         "div.anime_video_body_episodes_l",
    //         "div.anime_video_body_episodes_r",
    //         "iframe",
    //     ],
    // )?;
    let document = request_page(&episode_link).await?;

    // let document = Html::parse_document(&client);

    let iframe_link = document
        .select(&iframe_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .expect("iframe not found")
        .to_string();

    let prev_episode_div = document.select(&prev_episode_selector).next().unwrap();
    let prev_episode_link = prev_episode_div
        .select(&link_selector)
        .next()
        .map(|o| o.value().attr("href").unwrap().to_string());

    let next_episode_div = document.select(&next_episode_selector).next().unwrap();
    let next_episode_link = next_episode_div
        .select(&link_selector)
        .next()
        .map(|o| o.value().attr("href").unwrap().to_string());

    let episode_info: EpisodeInfo = EpisodeInfo {
        iframe_link: iframe_link,
        current_episode_link: episode_link,
        prev_episode_link: prev_episode_link,
        next_episode_link: next_episode_link,
    };

    Ok(episode_info)
}

/// scrapes a webpage using a headless browser. used when the
fn scrape_with_browser(
    url: &String,
    element_to_wait_for: Vec<&str>,
) -> Result<String, Box<dyn Error>> {
    // let browser = Browser::default()?;
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .expect("Failed to create launch options"),
    )
    .unwrap();
    println!("browser created");

    let tab = browser.new_tab()?;
    tab.navigate_to(&url)?;
    println!("{}", url);
    // insuring that all of the elements are loaded
    println!("waiting for elements to load...");
    tab.wait_for_element(element_to_wait_for.join(", ").as_str())?; //waits until the js DOM content has loaded the important elements
    let html = tab.get_content()?;

    Ok(html)
}
