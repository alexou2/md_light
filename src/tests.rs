use crate::online_md::*;
#[tokio::main]
#[test]
async fn tt() {
    let chaps = get_prev_and_next_chapters(
        "32a379d5-8bef-471b-9bfb-d52407d9ea84".to_string(),
        0.5,
        "142cab1a-005c-499b-9bdf-ff73cf5abd4a".to_string(),
        "en".to_string(),
    )
    .await;
    match chaps {
        Ok(c) => println!("{:#?}", c),
        Err(e) => println!("{}", e),
    }
}
