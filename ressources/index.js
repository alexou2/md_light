async function getAuhorChapters(authorID) {
    // for (let i in chapterID) {
    //     console.log(chapterID[i]);
    // }

    // let mangaList = await fetch("/author/feed", {
    //     method: "GET",
    //     // params: chapterID[0], // *GET, POST, PUT, DELETE, etc.
    //     body: JSON.stringify(chapterID)
    // })
    let mangaList = await fetch(`/author/${authorID}/feed`, )
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

async function get_feed(author){
    let mangaList = await fetch(`/author/feed?${author}`, )
    let manga = await mangaList.text();
    changeAuthorManga(manga)
}