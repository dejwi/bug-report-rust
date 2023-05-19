use actix_http::{header, StatusCode};
use actix_web::test;
use serde_json::Value as JsonValue;

use bug_report_backend::{
    models::{BugReportModel, BugReportStatus},
    schema::{
        AuthUserSchema, BugReportWithAuthorSchema, CreateBugReportSchema, UpdateBugReportSchema,
    },
};
mod common;

#[actix_web::test]
async fn integration() {
    let user = AuthUserSchema {
        username: "testuser".to_string(),
        password: "test".to_string(),
    };
    let app = common::get_test_app().await;

    // Create user
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&user)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    // Create user - should return that it already exists
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(&user)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CONFLICT);

    // Login
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&user)
        .to_request();

    let resp: JsonValue = test::call_and_read_body_json(&app, req).await;
    let token = resp["token"].as_str();

    assert!(token.is_some());

    let auth = (header::AUTHORIZATION, format!("Bearer {}", token.unwrap()));

    // Account information
    let req = test::TestRequest::get()
        .uri("/me")
        .insert_header(auth.clone())
        .to_request();
    let resp: JsonValue = test::call_and_read_body_json(&app, req).await;

    assert_eq!(
        resp["username"].as_str().unwrap_or("-EMPTY-"),
        user.username
    );
    assert!(resp["id"].as_str().is_some());
    assert!(resp["createdAt"].as_str().is_some());

    // Creating report
    let create_report = CreateBugReportSchema {
        title: "foo".to_string(),
        description: Some("bar".to_string()),
    };

    let req = test::TestRequest::post()
        .uri("/bug-reports")
        .insert_header(auth.clone())
        .set_json(&create_report)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::post()
        .uri("/bug-reports")
        .insert_header(auth.clone())
        .set_json(&create_report)
        .to_request();
    let report: BugReportWithAuthorSchema = test::call_and_read_body_json(&app, req).await;

    assert_eq!(report.title, create_report.title);
    assert_eq!(report.description, create_report.description);
    assert_eq!(report.author.username, user.username);

    // Get all reports
    let req = test::TestRequest::get().uri("/bug-reports").to_request();
    let reports: Vec<BugReportWithAuthorSchema> = test::call_and_read_body_json(&app, req).await;

    assert_eq!(reports.len(), 2);

    // Update report
    let update_report = UpdateBugReportSchema {
        description: Some("update".to_string()),
        title: Some("update".to_string()),
        status: Some(BugReportStatus::REVIEW),
    };

    let req = test::TestRequest::put()
        .uri(&format!("/bug-reports/{}", report.id.to_string()))
        .insert_header(auth.clone())
        .set_json(&update_report)
        .to_request();
    let updated: BugReportModel = test::call_and_read_body_json(&app, req).await;

    assert_eq!(updated.description, update_report.description);
    assert_eq!(updated.title, update_report.title.unwrap());
    assert_eq!(updated.status, update_report.status.unwrap());
}
