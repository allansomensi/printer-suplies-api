use crate::{
    handlers::{
        brand, printer, status,
        supplies::{drum, toner},
    },
    models::database::AppState,
};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest(
                    "/supplies",
                    Router::new()
                        // Toners
                        .route("/toner-count", get(toner::count_toners))
                        .route("/toner/:id", get(toner::search_toner))
                        .route(
                            "/toners",
                            get(toner::show_toners)
                                .post(toner::create_toner)
                                .put(toner::update_toner)
                                .delete(toner::delete_toner),
                        )
                        // Drums
                        .route("/drum-count", get(drum::count_drums))
                        .route("/drum/:id", get(drum::search_drum))
                        .route(
                            "/drums",
                            get(drum::show_drums)
                                .post(drum::create_drum)
                                .put(drum::update_drum)
                                .delete(drum::delete_drum),
                        ),
                )
                // Printers
                .route("/printer-count", get(printer::count_printers))
                .route("/printer/:id", get(printer::search_printer))
                .route(
                    "/printers",
                    get(printer::show_printers)
                        .post(printer::create_printer)
                        .put(printer::update_printer)
                        .delete(printer::delete_printer),
                )
                // Brands
                .route("/brand-count", get(brand::count_brands))
                .route("/brand/:id", get(brand::search_brand))
                .route(
                    "/brands",
                    get(brand::show_brands)
                        .post(brand::create_brand)
                        .put(brand::update_brand)
                        .delete(brand::delete_brand),
                )
                .route("/status", get(status::show_status)),
        )
        .with_state(state)
}
