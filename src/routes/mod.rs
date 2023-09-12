use actix_web::dev::HttpServiceFactory;

pub mod blog;
pub mod blog_category;
pub mod client;
pub mod employee;
pub mod job_listing;
pub mod product_feature;
pub mod continent;
pub mod country;
pub mod region;
pub mod city;
pub mod partner_vendor;
pub mod restaurant;
pub mod hotel;

pub fn hotel() -> impl HttpServiceFactory {
    (
        hotel::get_hotel_by_id_or_all,
        hotel::delete_hotel,
        hotel::create_hotel,
        hotel::update_hotel,
    )
}

pub fn restaurant() -> impl HttpServiceFactory {
    (
        restaurant::get_restaurant_by_id_or_all,
        restaurant::delete_restaurant,
        restaurant::create_restaurant,
        restaurant::update_restaurant,
    )
}

pub fn partner_vendor() -> impl HttpServiceFactory {
    (
        partner_vendor::get_partner_vendor_by_id_or_all,
        partner_vendor::delete_partner_vendor,
        partner_vendor::create_partner_vendor,
        partner_vendor::update_partner_vendor,
    )
}

pub fn city() -> impl HttpServiceFactory {
    (
        city::get_city_by_id_or_all,
        city::delete_city,
        city::create_city,
        city::update_city,
    )
}

pub fn region() -> impl HttpServiceFactory {
    (
        region::get_region_by_id_or_all,
        region::delete_region,
        region::create_region,
        region::update_region,
    )
}

pub fn country() -> impl HttpServiceFactory {
    (
        country::get_country_by_id_or_all,
        country::delete_country,
        country::create_country,
        country::update_country,
    )
}

pub fn client() -> impl HttpServiceFactory {
    (
        client::get_client_by_id_or_all,
        client::delete_client,
        client::create_client,
        client::update_client,
    )
}

pub fn employee() -> impl HttpServiceFactory {
    (
        employee::get_employee_by_id_or_all,
        employee::delete_employee,
        employee::create_employee,
        employee::update_employee,
    )
}

pub fn blog() -> impl HttpServiceFactory {
    (
        blog::create_blog,
        blog::get_blog_by_id_or_all,
        blog::get_featured_blogs,
        blog::update_blog,
        blog::delete_blog,
    )
}

pub fn job_listing() -> impl HttpServiceFactory {
    (
        job_listing::create_job_listing,
        job_listing::get_job_listing_by_id_or_all,
        job_listing::update_job_listing,
        job_listing::delete_job_listing,
    )
}

pub fn product_feature() -> impl HttpServiceFactory {
    (
        product_feature::create_product_feature,
        product_feature::get_product_feature_by_id_or_all,
        product_feature::update_product_feature,
        product_feature::delete_product_feature,
    )
}

pub fn blog_category() -> impl HttpServiceFactory {
    (
        blog_category::create_blog_category,
        blog_category::get_blog_category_by_id_or_all,
        blog_category::update_blog_category,
        blog_category::delete_blog_category,
    )
}

pub fn continent() -> impl HttpServiceFactory {
    (
        continent::create_continent,
        continent::get_continent_by_id_or_all,
        continent::update_continent,
        continent::delete_continent,
    )
}
