use crate::api_error::ApiError;
use crate::language::Language;
use crate::md_struct::*;
use crate::utills::*;
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::{from_str, Value};

const BASE_URL: &str = "https://api.mangadex.org";
const LIMIT: [(&str, i32); 1] = [("limit", 100)];
const CHAPTER_ORDERING: [(&str, &str); 1] = [("order[chapter]", "asc")];
const INCLUDE_TL_GROUP: [(&str, &str); 1] = [("includes[]", "scanlation_group")];

lazy_static! {
    pub static ref CLIENT: Client = Client::builder()
        .user_agent("Md_light")
        .build()
        .expect("unable to build Client");
}

/// sends a get request to the /ping endpoint of the api
pub async fn test_connection() -> Result<ServerStatus, ApiError> {
    // making a get request to the server
    let resp = request_with_agent(format!("{BASE_URL}/ping")).await;

    let reachable = resp.is_ok();

    // true only if the server response is "pong", otherwise the server is down
    let up = if reachable {
        let resp_content = &resp?;

        resp_content.as_str() == "pong"
    } else {
        false
    };

    let status = ServerStatus { up, reachable };
    Ok(status)
}

/// makes a web request
pub async fn request_with_agent(url: String) -> Result<String, ApiError> {
    let response = CLIENT.get(url).send().await?.text().await?;

    Ok(response)
}

/// requests manga chapters
async fn get_chaps(
    url: String,
    // offset: [(&str, i32); 1],
    offset: i32,
    language: Option<String>, // valid_req: &mut Arc<Mutex<bool>>,
) -> Result<Value, ApiError> {
    let mut req = CLIENT
        .get(url)
        // .header(reqwest::header::USER_AGENT, USER_AGENT)
        .query(&LIMIT)
        .query(&[("offset", offset)])
        // .query(&[("includeExternalUrl", "0")])
        .query(&CHAPTER_ORDERING)
        .query(&INCLUDE_TL_GROUP);

    if let Some(lang) = language {
        req = req.query(&[("translatedLanguage[]", lang)])
    }
    let response = req.send().await?;
    let response = response.text().await?;
    // let response = JSON_OFFLINE.to_string();
    // converting the text response into a json value
    let json_res_result = from_str(&response);
    let json_res: Value = match json_res_result {
        Ok(json) => json,
        Err(err) => {
            return Err(ApiError::Json(err));
        }
    };

    Ok(json_res)
}

/// gets the informations for the homepage
pub async fn get_md_homepage_feed(is_low_res: bool) -> Result<MdHomepageFeed, ApiError> {
    let popular_future = std::thread::spawn(move || get_popular_manga(is_low_res));
    let new_chap_future = std::thread::spawn(get_new_chapters);

    // builds the struct for the popular titles+ new chapters
    let homepage_feed = MdHomepageFeed {
        currently_popular: popular_future.join()?.await?,
        new_chapter_releases: new_chap_future.join()?.await?,
    };

    Ok(homepage_feed)
}

/// gets the new chapter releases for the homepage
pub async fn get_new_chapters() -> Result<Vec<NewChapters>, ApiError> {
    let mut new_chapters = Vec::new();
    // let url = "https://api.mangadex.org/chapter?includes[]=scanlation_group&translatedLanguage[]=en&translatedLanguage[]=de&contentRating[]=safe&contentRating[]=suggestive&contentRating[]=erotica&order[readableAt]=desc&limit=64&includes[]=cover_art";
    let url = "https://api.mangadex.org/chapter?includes[]=scanlation_group&contentRating[]=safe&contentRating[]=suggestive&contentRating[]=erotica&order[readableAt]=desc&limit=64&includes[]=cover_art";
    let resp = request_with_agent(url.to_string()).await?;
    // converts the api response to a json string and gets the data part of it
    let json_resp = parse_json(&resp).await?;
    let data = &json_resp["data"];

    // iterating over every chapter from the json file
    if let Some(new_chapter_list) = data.as_array() {
        // transforming the json data into an array
        for chapter in new_chapter_list {
            let attributes = &chapter["attributes"];
            let relationships = &chapter["relationships"];

            let chapter_id = &chapter["id"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;

            // if the chapter number is 0, then its name will be "Onshot"
            let chapter_number = attributes["chapter"]
                .remove_quotes()
                .unwrap_or("Oneshot".to_string());

            // sets the chapter name to the chapter number if the real chapter name is null
            let chapter_name = &attributes["title"]
                .remove_quotes()
                .unwrap_or(chapter_number.clone());

            let language = Language::from(&attributes["translatedLanguage"].remove_quotes());
            let page_number = attributes["pages"]
                .remove_quotes()
                .ok_or("error while removing quotes in page")?;

            let mut tl_group_id: String = String::new();
            let mut tl_group_name: String = String::new();
            let mut manga_id: String = String::new();

            // gets the infos about the tl group and the mangaID
            if let Some(relation) = relationships.as_array() {
                for rel in relation {
                    // matches the tl group or the manga_id and sets the value to the correct variable
                    match rel["type"].to_string().replace('"', "").as_str() {
                        "scanlation_group" => {
                            tl_group_id = rel["id"]
                                .remove_quotes()
                                .ok_or("error while removing quotes")?;
                            tl_group_name = rel["attributes"]["name"]
                                .remove_quotes()
                                .ok_or("error while removing quotes")?;
                        }
                        "manga" => {
                            manga_id = rel["id"]
                                .remove_quotes()
                                .ok_or("error while removing quotes")?
                        }
                        _ => continue,
                    }
                }
            }

            let new_chapter_data = NewChapters {
                chapter_name: chapter_name.to_owned(),
                chapter_number,
                language,
                manga_id,
                tl_group_id,
                tl_group_name,
                chapter_id: chapter_id.to_owned(),
                page_number,
            };
            new_chapters.push(new_chapter_data)
        }
    }

    Ok(new_chapters)
}

// gets the most popular mangas from the last month for the homepage feed
pub async fn get_popular_manga(is_low_res: bool) -> Result<Vec<PopularManga>, ApiError> {
    let mut popular_manga: Vec<PopularManga> = Vec::new();
    let formatted_time = get_offset_time();
    // formatting the request url to include the atrists/authors and the cover fileName
    let url = format!(
        r"https://api.mangadex.org/manga?includes[]=cover_art&includes[]=artist&includes[]=author&order[followedCount]=desc&contentRating[]=safe&contentRating[]=suggestive&hasAvailableChapters=true&createdAtSince={}",
        formatted_time
    );
    // let url = format!(
    //         r"https://api.mangadex.org/manga?includes[]=cover_art&includes[]=artist&includes[]=author&order[followedCount]=desc&hasAvailableChapters=true&createdAtSince={}",
    //         formatted_time
    //     );

    // doing the get request to the api and transforming it into a json object
    let resp = request_with_agent(url).await?;
    let json_resp = parse_json(&resp).await?;

    // transforming the json into an array in order to get all of the search results
    if let Some(response_data) = json_resp["data"].as_array() {
        for manga in response_data {
            let title = manga["attributes"]["title"]
                .as_object()
                .and_then(|obj| obj.values().next())
                .ok_or("error while getting manga title")?
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let manga_id = &manga["id"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let cover = get_manga_cover(manga_id, manga, is_low_res)?;

            // creating the search result for each popular manga
            let manga_instance = PopularManga {
                title,
                cover,
                id: manga_id.clone(),
            };

            popular_manga.push(manga_instance);
        }
    }

    Ok(popular_manga)
}

/// searches for a manga. Can also use parameters to search for authors or more
pub async fn search_manga(
    title: Option<String>,
    params: Option<[(&str, String); 1]>,
    is_low_res: bool,
) -> Result<Vec<ShortMangaInfo>, ApiError> {
    let mut search_results: Vec<ShortMangaInfo> = Vec::new();
    // sending the get request for the search
    let url = format!(
        "{}/manga?includes[]=cover_art&includes[]=author&includes[]=artist",
        BASE_URL
    ); // formatting the correct url for the api endpoint
    let title_param = [("title", title)]; // setting the parameters of the search

    let resp = CLIENT
        .get(url)
        .query(&title_param)
        .query(&params)
        // .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .text()
        .await?;
    // converting the response string into a json object
    // let json_resp: Value = serde_json::from_str(&resp)?;
    let json_resp = parse_json(&resp).await?;

    // if the api response is an array, add every manga to the search_results vector
    if let Some(response_data) = json_resp["data"].as_array() {
        for manga in response_data {
            let attributes = &manga["attributes"];

            // getting eery necessary info from the manga
            let manga_id = &manga["id"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;

            let title: String = attributes["title"]
                .as_object()
                .and_then(|obj| obj.values().next())
                .ok_or("error while getting title")?
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let status = &attributes["status"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let original_language = Language::from(&attributes["originalLanguage"].remove_quotes());
            // let available_languages = attributes["availableTranslatedLanguages"]
            //     .as_array()
            //     .ok_or("error while getting translated languages options")?;
            let available_languages =
                Language::to_language_vec(attributes["availableTranslatedLanguages"].as_array());
            let cover = get_manga_cover(manga_id, manga, is_low_res)?;
            let description = &attributes["description"]["en"]
                .remove_quotes()
                .unwrap_or("No description".to_string());
            // creating the struct instnce containing all of the usefull ionfos about the manga
            let manga_attributes = ShortMangaInfo {
                title,
                id: manga_id.clone(),
                cover,
                status: status.clone(),
                original_language,
                translated_languages: available_languages,
                description: description.clone(),
            };
            search_results.push(manga_attributes)
        }
    }
    Ok(search_results)
}

/// searches for an author
pub async fn search_author(query: String) -> Result<Vec<AuthorInfo>, ApiError> {
    let url = format!("{BASE_URL}/author?name={query}");

    // does the request and converts it to json
    let resp = request_with_agent(url).await?;
    // let json_resp: Value = from_str(&resp)?;
    let json_resp = parse_json(&resp).await?;

    //converting the response data into an array
    let author_list = json_resp["data"]
        .as_array()
        .ok_or("unable to convert author search to array")?;

    let mut author_info_list = vec![];
    // gets the infos for each author in the response
    for author in author_list {
        let id = author["id"]
            .remove_quotes()
            .ok_or("unable to remove quotes")?;
        let name = author["attributes"]["name"]
            .remove_quotes()
            .ok_or("unable to remove quotes")?;

        let title_list = author["relationships"]
            .as_array()
            .ok_or("unable to convert author search to array")?;

        let mut title_id_list = vec![];

        // gets all of the author's titles
        for title in title_list {
            let title_id = title["id"]
                .remove_quotes()
                .ok_or("unable to remove quotes")?;
            title_id_list.push(title_id)
        }

        let author_info = AuthorInfo {
            name,
            id,
            titles_id: title_id_list,
        };
        author_info_list.push(author_info);
    }
    Ok(author_info_list)
}

// gets the cover from the json the request url needs to have includes[]=cover_art for this function to work
pub fn get_manga_cover(
    manga_id: &String,
    manga_json: &Value,
    is_low_res: bool,
) -> Result<String, ApiError> {
    //uses the low quality images if the low auality argument is given
    let quality = match is_low_res {
        false => 512,
        true => 256,
    };

    let mut thumbnail = String::new();
    if let Some(manga_cover) = manga_json["relationships"].as_array() {
        for i in manga_cover {
            if i["type"] == "cover_art" {
                let cover_id = i["attributes"]["fileName"].to_string();
                let cover_link =
                    format!("https://mangadex.org/covers/{manga_id}/{cover_id}.{quality}.jpg")
                        .replace('"', "");
                thumbnail = cover_link;
                break; //breaks the loop if the cover is found
            }
        }
    }
    Ok(thumbnail)
}

// gets the manga info and chapters
pub async fn get_manga_info(manga_id: String, is_low_res: bool) -> Result<MangaInfo, ApiError> {
    // calls the function to get chapters for a faster page loading
    // let id_clone = manga_id.clone();
    // let manga_chapters_future = get_manga_chapters(id_clone, None);

    let url = format!(
        "{}/manga/{}?includes[]=author&includes[]=artist&includes[]=cover_art",
        BASE_URL, &manga_id
    );
    // calling the function to make the request to the api
    let resp = request_with_agent(url.clone()).await?;
    // parsing the api response into a json
    // let json_resp: Value = from_str(&resp)?;
    let json_resp = parse_json(&resp).await?;
    // separating the json response to make it easier to access items
    let data = &json_resp["data"];
    let attributes = &data["attributes"];

    // gets all of the infos about manga
    let manga_name = attributes["title"]
        .as_object()
        .and_then(|obj| obj.values().next())
        .ok_or("error while getting title")?
        .remove_quotes()
        .ok_or("error while removing quotes in the manga name")?;
    let cover = get_manga_cover(&manga_id, data, is_low_res)?;
    let status = &attributes["status"]
        .remove_quotes()
        .ok_or("error while removing quotes in the status")?;
    let original_language = Language::from(&attributes["originalLanguage"].remove_quotes());
    let description = &attributes["description"]["en"]
        .remove_quotes()
        .unwrap_or("N/a".to_string());
    let year = &attributes["year"].as_i64();

    let mut author_list: Vec<Author> = Vec::new();
    let mut tag_list = Vec::new();
    let mut translated_language_list = Vec::new();

    // transforming the json part containing the authors into an array
    let author_json = data["relationships"]
        .as_array()
        .ok_or("authors is not an array")?;
    // gets the list pf authors involved in the manga
    for author in author_json {
        // breaks the loop if the data isn't about an author/drawer
        match author["type"].to_string().replace('"', "").as_str() {
            "author" | "artist" => {
                let author_name = &author["attributes"]["name"]
                    .remove_quotes()
                    .ok_or("error while removing quotes in the author name")?;
                let author_id = &author["id"]
                    .remove_quotes()
                    .ok_or("error while removing quotes for the author ID")?;
                let role = &author["type"]
                    .remove_quotes()
                    .ok_or("error while removing quotes in the author role")?;

                // the author/artist instance
                let author_instance = Author {
                    author_name: author_name.to_string(),
                    author_id: author_id.to_string(),
                    role: role.to_string(),
                };
                author_list.push(author_instance)
            }
            _ => break,
        }
    }

    // getting the tags
    let tag_json = attributes["tags"]
        .as_array()
        .ok_or("tags is not an array")?;
    for tag in tag_json {
        let tag_name = &tag["attributes"]["name"]["en"]
            .remove_quotes()
            .ok_or(format!(
                "error while removing quotes in the tags: {}",
                tag["attributes"]["name"]["en"]
            ))?;
        tag_list.push(tag_name.clone());
    }

    // getting the translation options
    let translation_options_json = attributes["availableTranslatedLanguages"]
        .as_array()
        .ok_or("translated_languages is not an array")?;
    for language in translation_options_json {
        let translation = Language::from(language.remove_quotes());
        translated_language_list.push(translation);
    }

    // building the struct with all of the manga's informations+ chapters
    let manga_info = MangaInfo {
        manga_name,
        manga_id,
        author: author_list,
        tags: tag_list,
        cover,
        status: status.clone(),
        original_language,
        translated_languages: translated_language_list,
        year: *year,
        description: description.clone(),
    };
    Ok(manga_info)
}

// gets all of the manga's chapters for the manga info page
// returns a vector that contains both errors and chapters
pub async fn get_manga_chapters(
    manga_id: String,
    language: Option<String>,
    offset: i32,
    // ) -> Result<Vec<Result<Chapter, ApiError>>, ApiError> {
) -> Result<MangaChapters, ApiError> {
    let url = format!("{}/manga/{}/feed", BASE_URL, manga_id);

    let chapter_json = get_chaps(url, offset, language).await?;

    let mut json_list: Vec<Value> = vec![]; // a list containing the json data about the chapters

    // loops through all of the request
    // for chap in chapter_json {
    //     // skips the request if there is an error
    //     let chap = match chap {
    //         Ok(e) => e,
    //         Err(_) => {
    //             continue;
    //         }
    //     };

    // divides the json data into a list of json elements representing chapters
    let list = chapter_json["data"]
        .as_array()
        .ok_or("unable to convert chapters to array")
        .unwrap();

    // json_list.append(&mut list.clone());

    for i in list {
        json_list.push(i.to_owned());
    }
    // }
    let ch_number = from_str::<i32>(&chapter_json["total"].to_string())
        .expect("cant't get total chapter number");

    let mut chapter_list: Vec<Result<Chapter, ApiError>> = Vec::new();
    // let chapter_json = data.as_array().ok_or("there are no chapters")?; // transforming the json into an array

    for chapter in json_list {
        // skips the chapter if it has an error
        // let chapter = chap;
        // match chap {
        //     Ok(e) => chapter = e,
        //     Err(v) => {
        //         chapter_list.push(Err(v));
        //         continue;
        //     }
        // };

        let attributes = &chapter["attributes"];
        let chapter_number = &attributes["chapter"];

        let chapter_number = chapter_number
            .remove_quotes()
            .unwrap_or("Oneshot".to_string()); // if there is no chapter number, set the chapter as a Oneshot

        let chapter_name = attributes["title"].remove_quotes();
        let language = Language::from(&attributes["translatedLanguage"].remove_quotes());
        // .ok_or(format!(
        //     "error while removing quotes in the chapter language {}",
        //     attributes["translatedLanguage"]
        // ));
        let chapter_id = chapter["id"]
            .remove_quotes()
            .ok_or("error while removing quotes in the chapter ID")
            .expect("can't get chapterID");

        // getting the translator groups
        let mut tl_group: Vec<TlGroup> = Vec::new();

        let relationships = chapter["relationships"]
            .as_array()
            .ok_or("Unable to convert chapter_relationships into an array");

        if relationships.is_ok() {
            let relationships = relationships.unwrap();
            for relation in relationships {
                if relation["type"] == "scanlation_group" {
                    let group_name = &relation["attributes"]["name"]
                        .remove_quotes()
                        .ok_or("error while removing tl_name quotes");
                    let group_id = relation["id"]
                        .remove_quotes()
                        .ok_or("unable to remove quotes in tl_group id");
                    // cheks if the group's ID or name is an error
                    if group_name.is_ok() && group_id.is_ok() {
                        let name = group_name.as_ref().unwrap();
                        let id = group_id.unwrap();

                        let group = TlGroup {
                            name: name.clone(),
                            id: id.clone(),
                        };
                        tl_group.push(group);
                    }
                }
            }
        }

        let chapter_instance = Ok(Chapter {
            chapter_name,
            chapter_number,
            language,
            tl_group,
            chapter_id,
        });
        chapter_list.push(chapter_instance)
    }

    let ret = MangaChapters {
        chapters: chapter_list,
        total: ch_number,
    };

    Ok(ret)
}

pub async fn get_chapter_pages(chapter_id: String) -> Result<ChapterPage, ApiError> {
    let url = format!("{}/at-home/server/{}", BASE_URL, chapter_id);
    // let resp = reqwest::get(&url).await?.text().await?;
    let resp = request_with_agent(url).await?;

    // let json_resp: Value = from_str(&resp)?;
    let json_resp = parse_json(&resp).await?;
    let chapter_hash = json_resp["chapter"]["hash"]
        .remove_quotes()
        .ok_or("can't get chapter hash")?;

    let pages_json = json_resp["chapter"]["data"]
        .as_array()
        .ok_or("there are no pages")?; //transforming the response string into a json object

    let mut page_list: Vec<String> = Vec::new();
    for page in pages_json {
        let mut page_link = page
            .remove_quotes()
            .ok_or("error while removing quotes")?
            .to_string();
        page_link = format!(
            "https://uploads.mangadex.org/data/{}/{}",
            chapter_hash, page_link
        );
        page_list.push(page_link)
    }
    let chapter = ChapterPage {
        name: "ch".to_string(),
        pages: page_list,
    };
    Ok(chapter)
}

pub async fn get_author_infos(id: String) -> Result<AuthorInfo, ApiError> {
    let url = format!("{}/author/{}", BASE_URL, id);
    let resp = request_with_agent(url).await?;
    // let json_resp: Value = from_str(&resp)?;
    let json_resp = parse_json(&resp).await?;

    let data = &json_resp["data"];

    let name = &data["attributes"]["name"]
        .remove_quotes()
        .ok_or("error in author name")?;
    let titles_json = data["relationships"]
        .as_array()
        .ok_or("error in author's manga list")?;

    let mut titles_id = Vec::<String>::new();
    for id in titles_json {
        titles_id.push(
            id["id"]
                .remove_quotes()
                .ok_or("unable to remove quotes from title id")?,
        );
    }

    let author_info = AuthorInfo {
        name: name.clone(),
        id,
        titles_id,
    };

    Ok(author_info)
}

// parses the json response from the api and returns an error if it is invalid
async fn parse_json(response: &str) -> Result<Value, ApiError> {
    let json_resp = from_str(response);
    // checks if the response is of type error
    let json_success: Value = match json_resp {
        Ok(v) => v,
        Err(e) => return Err(ApiError::Json(e)),
    };

    // Ok(json_success)
    let result = json_success["result"].to_owned();
    match result.to_string().as_str() {
        r#""error""# => Err(ApiError::ApiPageNotFound404),
        r#""ok""# => Ok(json_success),
        _ => Err(ApiError::ApiResponseError),
    }
}

/// returns the previous and next chapter
pub async fn get_prev_and_next_chapters(
    chapter_id: String,
    chapter_number: &str,
    manga_id: String,
    language: String,
) -> Result<CurrentChapter, ApiError> {
    let offset = get_offset_from_f32(chapter_number);
    let chapters = get_manga_chapters(manga_id, Some(language), offset).await?;
    let mut index = 0;

    for i in 0..chapters.chapters.len() {
        // if chapters.chapters[i]
        let ch = chapters.chapters[i].as_ref().unwrap();
        if ch.chapter_id == chapter_id {
            index = i;
            break;
        }
    }
    println!("index: {index} ||| len: {}", chapters.chapters.len());
    let mut prev = None;
    let mut next = None;

    if !chapters.chapters.is_empty() {
        if index != chapters.chapters.len() - 1 {
            next = Some(chapters.chapters[index + 1].as_ref().unwrap().clone())
        }
        if index > 0 {
            prev = Some(chapters.chapters[index - 1].as_ref().unwrap().clone())
        }
    }
    Ok(CurrentChapter {
        prev,
        next,
        curr_chapter_name: chapters.chapters[index]
            .as_ref()
            .unwrap()
            .chapter_name
            .clone(),
        curr_chapter_number: chapter_number.parse().unwrap_or(0.0),
    })
}

/// returns the offset required to get the previous and next chapters
fn get_offset_from_f32(number: &str) -> i32 {
    if number == "Oneshot" {
        return 0;
    }
    let number: f32 = from_str(number).unwrap();
    let mut offset = (number - 10.0) as i32;
    if offset < 0 {
        offset = 0;
    }
    offset
}
