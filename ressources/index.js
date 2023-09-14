async function getAuhorChapters(authorID) {
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
  }