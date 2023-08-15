use actix_web::web::Json;
use std::time::SystemTime;
use crate::db::insert;
use crate::data_types::structs::NewContinent;

pub async fn execute(new_continent: Json<NewContinent>) {
    let name: &String = &new_continent.name;
    let slug: &String = &new_continent.slug;
    let description_long: &String = &new_continent.description_long;
    let description_short: &String = &new_continent.description_short;
    let image_link: &String = &new_continent.image_link;
    let thumbnail_link: &String = &new_continent.thumbnail_link;
    let special_offer_image_link: &String = &new_continent.special_offer_image_link;
    let video_link: &String = &new_continent.video_link;
    let gallery: &Vec<String> = &new_continent.gallery;
    let tags: &String = &new_continent.tags;

    insert("continent",
        vec!["name", "slug", "description_long", "description_short", "image_link", "thumbnail_link", "special_offer_image_link", "video_link", "gallery", "tags"],
        Some(&[
            &name,
            &slug,
            &description_long,
            &description_short,
            &image_link,
            &thumbnail_link,
            &special_offer_image_link,
            &video_link,
            &gallery,
            &tags,
        ]
    )).await;
}
