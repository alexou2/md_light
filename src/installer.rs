use std::fs::write;

pub fn install_ressources(){

}

const JS_FILE:&'static str = r#"async function getAuhorChapters(authorID) {
    // for (let i in chapterID) {
    //     console.log(chapterID[i]);
    // }

    // let mangaList = await fetch("/author/feed", {
    //     method: "GET",
    //     // params: chapterID[0], // *GET, POST, PUT, DELETE, etc.
    //     body: JSON.stringify(chapterID)
    // })
    let mangaList = await fetch(`/author/${authorID}/feed`,)
    let manga = await mangaList.text();
    console.log(await mangaList)
    changeAuthorManga(manga)

}
function changeAuthorManga(content) {
    console.log("content", content)
    let manga_div = document.getElementById("author_manga")
    console.log(manga_div.innerHTML);
    manga_div.innerHTML = content
}

async function get_feed(author) {
    let mangaList = await fetch(`/author/feed?${author}`,)
    let manga = await mangaList.text();
    changeAuthorManga(manga)
}
// changes the page link to seatch for a manga
function search() {
    let input = document.getElementById("search_box").value;

    if (input != "") {
        input = `/search/${input}`
        console.log(input)
        window.location.href = input

    } else {
        alert("empty search query")
    }
}

function goFullscreen() {
    // document.getElementsByClassName("entire").requestFullscreen();
  
    var elem = document.documentElement;
  
    /* View in fullscreen */
    if (elem.requestFullscreen) {
      elem.requestFullscreen();
    } else if (elem.webkitRequestFullscreen) { /* Safari */
      elem.webkitRequestFullscreen();
    } else if (elem.msRequestFullscreen) { /* IE11 */
      elem.msRequestFullscreen();
    }
  
    // exit fullscreen
    if (document.exitFullscreen) {
      document.exitFullscreen();
    } else if (document.webkitExitFullscreen) { /* Safari */
      document.webkitExitFullscreen();
    } else if (document.msExitFullscreen) { /* IE11 */
      document.msExitFullscreen();
    }
  }"#;

const CSS_FILE:&'static str = r#"img.chapter_page {
    display: block;
    margin-left: auto;
    margin-right: auto;
    max-width: 100%;
  }
  
  body {
    background-color: black;
  }
  
  a.chapter_link:link {
    color: rgb(255, 182, 0);
  }
  
  /* visited link */
  a.chapter_link:visited {
    color: grey;
  }
  
  a {
    color: aqua;
  }
  
  body {
    color: aqua;
  }
  
  .chapter_item {
    display: flex;
    /* align-content: basseline; */
    justify-content: space-between;
    margin: 1em;
  }
  
  .top_bar {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    justify-content: space-between;
    align-items: baseline;
  }
  
  
  #search_box {
    /* height: 2em;
    font-size: 1.5em;
    border-radius: 5em;
    height: 100%; */
    /* width: 100%; */
      padding: 10px;
      box-sizing: border-box;
      margin-bottom: 10px;
      border: none;
      outline: none;
      border: 1px solid grey;
      font-size: 15px;
      border-radius: 5px;
  }
  .search_bar{
    /* height: 1.7em;
    font-size: 1.5em;
    border-radius: 5em;
    background-color: blue; */
  
  }
  
  
  /* .manga_info {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
  }
  
  .author_list {
    align-self: flex-end;
  } */
  .description{
    max-width: 50%;
  }
  .author_list{
    color: rgb(255, 183, 0);
  }"#;