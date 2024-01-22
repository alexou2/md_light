async function getAuhorChapters(authorID) {
  let mangaList = await fetch(`/author/${authorID}/feed`,);
  let manga = await mangaList.text();
  console.log(await mangaList)
  changeAuthorManga(manga)

}
// loads the mangas created by the author
function changeAuthorManga(content) {
  console.log("content", content)
  let manga_div = document.getElementById("works")
  // console.log(manga_div.innerHTML);
  manga_div.innerHTML = content
}
//gets the titles from the author
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

// function to go fullscreen
function goFullscreen() {
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
async function fetch_chapter(mangaID, offset) {
  let chapterPlacement = document.getElementById("chapter_list");
  try{
  document.getElementById("chapter").innerHTML = `<div class="loading"></div>`
// chapterPlacement.innerHTML = ""
  }catch{}
  console.log(`url:   /chapters/${mangaID}?offset=${offset}`);
  let html = await fetch(`/chapters/${mangaID}?offset=${offset}`);
  let resp = await html.text()
  chapterPlacement.innerHTML = resp
}