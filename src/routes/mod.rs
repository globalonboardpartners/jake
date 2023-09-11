use actix_web::dev::HttpServiceFactory;

pub mod blog;
pub mod blog_category;
pub mod client;
pub mod employee;
pub mod job_listing;
pub mod product_feature;

pub fn client() -> impl HttpServiceFactory {
    (
        client::get_all_clients,
        client::get_client_by_id,
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
        blog_category::get_all_blog_categories,
        blog_category::get_blog_category_by_id,
        blog_category::update_blog_category,
        blog_category::delete_blog_category,
    )
}
