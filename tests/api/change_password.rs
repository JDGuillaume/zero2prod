use crate::helpers::{assert_is_redirect_to, spawn_app};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_change_password().await;

    // Asset
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act
    let response = app
        .post_change_password(&serde_json::json!({
          "current_password": Uuid::new_v4().to_string(),
          "new_password": &new_password,
          "new_password_check": &new_password,
        }))
        .await;

    // Asset
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();

    // Act #1: Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // Act #2: Try to change the password.
    let response = app
        .post_change_password(&serde_json::json!({
          "current_password": &app.test_user.password,
          "new_password": &new_password,
          "new_password_check": &another_new_password,
        }))
        .await;

    // Assert #2: Confirm redirection to dashboard.
    assert_is_redirect_to(&response, "/admin/password");

    // Act #3: Follow the redirect.
    let html_page = app.get_change_password_html().await;

    // Assert #3: Check to see that error message is displayed.
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // Act #1: Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // Act #2: Try to change the password.
    let response = app
        .post_change_password(&serde_json::json!({
          "current_password": &wrong_password,
          "new_password": &new_password,
          "new_password_check": &new_password,
        }))
        .await;

    // Assert #2: Confirm redirection to dashboard.
    assert_is_redirect_to(&response, "/admin/password");

    // Act #3: Follow the redirect.
    let html_page = app.get_change_password_html().await;

    // Assert #3: Check to see that error message is displayed.
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

#[tokio::test]
async fn passwords_must_be_longer_than_12_characters() {
    // Arrange
    let app = spawn_app().await;
    let new_password = "a".repeat(12);

    // Act #1: Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // Act #2: Try to change the password.
    let response = app
        .post_change_password(&serde_json::json!({
          "current_password": &app.test_user.password,
          "new_password": &new_password,
          "new_password_check": &new_password,
        }))
        .await;

    // Assert #2: Confirm redirection to dashboard.
    assert_is_redirect_to(&response, "/admin/password");

    // Act #3: Follow the redirect.
    let html_page = app.get_change_password_html().await;

    // Assert #3: Check to see that error message is displayed.
    assert!(html_page.contains("<p><i>Passwords should be longer than 12 characters but shorter than 129 characters.</i></p>"));
}

#[tokio::test]
async fn passwords_must_be_shorter_than_129_characters() {
    // Arrange
    let app = spawn_app().await;
    let new_password = "a".repeat(129);

    // Act #1: Login
    app.post_login(&serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    }))
    .await;

    // Act #2: Try to change the password.
    let response = app
        .post_change_password(&serde_json::json!({
          "current_password": &app.test_user.password,
          "new_password": &new_password,
          "new_password_check": &new_password,
        }))
        .await;

    // Assert #2: Confirm redirection to dashboard.
    assert_is_redirect_to(&response, "/admin/password");

    // Act #3: Follow the redirect.
    let html_page = app.get_change_password_html().await;

    // Assert #3: Check to see that error message is displayed.
    assert!(html_page.contains("<p><i>Passwords should be longer than 12 characters but shorter than 129 characters.</i></p>"));
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();

    // Act #1: Login
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act #2: Change Password
    let response = app
        .post_change_password(&serde_json::json!({
          "current_password": &app.test_user.password,
          "new_password": &new_password,
          "new_password_check": &new_password,
        }))
        .await;

    assert_is_redirect_to(&response, "/admin/password");

    // Act #3: Follow the redirect.
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));

    // Act #4: Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act #5: Follow the redirect.
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));

    // Act #6: Login using the new password.
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &new_password,
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
