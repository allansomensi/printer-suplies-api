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
                        .route(
                            "/toners",
                            get(toner::show_toners)
                                .post(toner::create_toner)
                                .delete(toner::delete_toner),
                        )
                        // Drums
                        .route("/drum-count", get(drum::count_drums))
                        .route(
                            "/drums",
                            get(drum::show_drums)
                                .post(drum::create_drum)
                                .delete(drum::delete_drum),
                        ),
                )
                // Printers
                .route("/printer-count", get(printer::count_printers))
                .route(
                    "/printers",
                    get(printer::show_printers)
                        .post(printer::create_printer)
                        .delete(printer::delete_printer),
                )
                // Brands
                .route("/brand-count", get(brand::count_brands))
                .route(
                    "/brands",
                    get(brand::show_brands)
                        .post(brand::create_brand)
                        .delete(brand::delete_brand),
                )
                .route("/status", get(status::show_status)),
        )
        .with_state(state)
}
