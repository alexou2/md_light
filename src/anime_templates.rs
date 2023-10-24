// all of the templates for the different pages of the app

use crate::anime_scraper::*;
use std::error::Error;

// returns the html code that wil be used for the homepage
pub fn render_homepage(data: Vec<NewEpisodeReleases>) -> String {
    let elements_to_insert = generate_new_releases_list(data);

    // the base of the html page, whithout the new episode list
    let mut template = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        
          <head>
            <meta charset="UTF-8" />
            <link rel="stylesheet" href="styles.css" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>AniCli</title>
            <script type="module" src="/main.js" defer></script>
            <script src="/main.js"></script>
            <style>
              .logo.vanilla:hover {{
                filter: drop-shadow(0 0 2em #ffe21c);
              }}
            </style>
          </head>

<body>
<img class="icon" src="some_guy.png" alt="image not loaded" onclick="loadHomepage()">
    <h1 id="pageTitle">New Episodes</h1>
    <div id="content">
    <div class="animeContainer" id="animeContainer">    
    <div class="animeList" id="animeList">
        {}
    </div>
    </div>
    </div>
</body>

</html>"#,
        elements_to_insert.join(" ")
    );
    // removes unwanted characters from the template string
    template = template
        .replace("[\"", "")
        .replace("\"]", "")
        .replace("\\\"", "\"")
        .replace("\", \"", "");
    return template;
}

// creates the episode list used in the homepage
fn generate_new_releases_list(data: Vec<NewEpisodeReleases>) -> Vec<String> {
    // the vector used to store the html snippets for each of the new episodes releases
    let mut elements_to_insert: Vec<String> = Vec::new();

    // generates the html for every new episode release for the main page
    for i in 0..data.len() - 1 {
        // creates each anime element to insert in the main html
        let image_link = format!(
            r#"
        <div class="animeItem" id="animeItem">
        <img class="animeThumbnail" id="animeThumbnail" src="{thumbnail}"
        onclick="loadAnimeInfoPage(`{anime_link}`, `{thumbnail}`, `{name}`)" title="{name}"><br>
        {name}
        <div class="newEpisode" onclick="watchEpisode(`{episode_link}`)">Episode {episode_number}</div>
        </div>"#,
            thumbnail = data[i].thumbnail,
            anime_link=data[i].link_to_anime_page,
            // data[i].thumbnail,
            name=data[i].anime_name,
            // data[i].anime_name,
            // data[i].anime_name,
           episode_link= data[i].link_to_last_episode,
            // data[i].anime_name,
           episode_number= data[i].last_episode_number
        )
        .replace("\n", "");
        // adds the element created to the vector
        elements_to_insert.push(image_link);
    }
    elements_to_insert
}

// returns the entire html code for the anime description page
pub fn render_anime_description_page(data: AnimeDescription) -> String {
    let elements_to_insert = generate_episode_list(data.episodes_list);
    // the base of the anime infos
    let mut template = format!(
        r#"
    <img class="animeThumbnail" id="animeThumbnail" src="{thumbnail}" onclick = "loadHomepage()">{infos}
    <div class="animeContainer" id="animeContainer">    
    <div class="animeList" id="animeList">
        {episodes}
    </div>
    </div>"#,
        // fills the blank spots with the thumbnail, infos and episodes
        thumbnail = data.thumbnail,
        infos = data.infos.join("<br><br>"),
        episodes = elements_to_insert.join(" ")
    );
    template = template
        .replace("[\"", "")
        .replace("\"]", "")
        .replace("\\\"", "\"")
        .replace("\", \"", "");
    println!("{}", template);
    return template;
}

fn generate_episode_list(data: Vec<AnimeEpisode>) -> Vec<String> {
    let mut episode_list: Vec<String> = Vec::new();
    for i in data {
        let episode = format!(
            r#"<div class="episode" onclick="watchEpisode(`{link}`)">{name}</div> "#,
            link = i.link_to_episode,
            name = i.episode_name
        );
        episode_list.push(episode)
    }
    episode_list
}

pub fn render_episode_page(data: EpisodeInfo) -> String {
    let template = format!(
        r#"<h1>aniem</h1>
    <h3 style="text-align: center;">episode</h3>
<div class="video"><iframe style="width: 80%; height: fit-content;" src="{iframe}"></iframe></div>
<span class="prev-episode" style="text-align: left;">prev episode</span>
<span class="next-episode" style="left: 0;">next episode</span>
</div>"#,
        iframe = data.iframe_link
    );
    template
}

pub fn return_error_page(error_code: Box<dyn Error>) -> String {
    let error_page = format!(
        r#"<!DOCTYPE html>
     <html lang="en">
     
     <head>
         <meta charset="UTF-8" />
         <link rel="stylesheet" href="styles.css" />
         <meta name="viewport" content="width=device-width, initial-scale=1.0" />
         <title>AniCli</title>
         <script src="main.js"></script>
         <style>
         </style>
     </head>
     
     <body>
         
         <h1>Oops, looks like there is an error</h1>
         <img class="errorIcon" src="error.png" onclick="loadHomepage(`true`)" title="refresh">
         
     <div class="errorMessage">{error}</div>
     </body>
     
     </html>"#,
        error = error_code
    )
    .to_string();
    println!("{}", error_code);
    error_page
}
