use actix_web::http::StatusCode;
use assert_json_diff::assert_json_eq;
use serde_json::{json, Value};

mod common;

#[actix_rt::test]
async fn create_index_with_name() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index

    let body = json!({
        "name": "movies",
    });

    let (res1_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 8);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid.len(), r1_uid.len());
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[actix_rt::test]
async fn create_index_with_uid() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index

    let body = json!({
        "uid": "movies",
    });

    let (res1_value, status_code) = server.create_index(body.clone()).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid, "movies");
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 1.5 verify that error is thrown when trying to create the same index

    let (response, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
    assert_eq!(
        response["errorCode"].as_str().unwrap(),
        "index_already_exists"
    );

    // 2 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid, r1_uid);
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[actix_rt::test]
async fn create_index_with_name_and_uid() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index

    let body = json!({
        "name": "Films",
        "uid": "fr_movies",
    });
    let (res1_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "Films");
    assert_eq!(r1_uid, "fr_movies");
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid, r1_uid);
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[actix_rt::test]
async fn rename_index() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index

    let body = json!({
        "name": "movies",
        "uid": "movies",
    });

    let (res1_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 6);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Update an index name

    let body = json!({
        "name": "TV Shows",
    });

    let (res2_value, status_code) = server.update_index(body).await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_object().unwrap().len(), 5);
    let r2_name = res2_value["name"].as_str().unwrap();
    let r2_uid = res2_value["uid"].as_str().unwrap();
    let r2_created_at = res2_value["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value["updatedAt"].as_str().unwrap();
    assert_eq!(r2_name, "TV Shows");
    assert_eq!(r2_uid, r1_uid);
    assert_eq!(r2_created_at, r1_created_at);
    assert!(r2_updated_at.len() > 1);

    // 3 - Check the list of indexes

    let (res3_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res3_value.as_array().unwrap().len(), 1);
    assert_eq!(res3_value[0].as_object().unwrap().len(), 5);
    let r3_name = res3_value[0]["name"].as_str().unwrap();
    let r3_uid = res3_value[0]["uid"].as_str().unwrap();
    let r3_created_at = res3_value[0]["createdAt"].as_str().unwrap();
    let r3_updated_at = res3_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r3_name, r2_name);
    assert_eq!(r3_uid.len(), r1_uid.len());
    assert_eq!(r3_created_at.len(), r1_created_at.len());
    assert_eq!(r3_updated_at.len(), r2_updated_at.len());
}

#[actix_rt::test]
async fn delete_index_and_recreate_it() {
    let mut server = common::Server::with_uid("movies");

    // 0 - delete unexisting index is error

    let (response, status_code) = server.delete_request("/indexes/test").await;
    assert_eq!(status_code, 404);
    assert_eq!(&response["errorCode"], "index_not_found");

    // 1 - Create a new index

    let body = json!({
        "name": "movies",
        "uid": "movies",
    });

    let (res1_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 6);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid.len(), r1_uid.len());
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());

    // 3- Delete an index

    let (_res2_value, status_code) = server.delete_index().await;

    assert_eq!(status_code, 204);

    // 4 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 0);

    // 5 - Create a new index

    let body = json!({
        "name": "movies",
    });

    let (res1_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 8);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 6 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;
    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_name = res2_value[0]["name"].as_str().unwrap();
    let r2_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r2_name, r1_name);
    assert_eq!(r2_uid.len(), r1_uid.len());
    assert_eq!(r2_created_at.len(), r1_created_at.len());
    assert_eq!(r2_updated_at.len(), r1_updated_at.len());
}

#[actix_rt::test]
async fn check_multiples_indexes() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create a new index

    let body = json!({
        "name": "movies",
    });

    let (res1_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res1_value.as_object().unwrap().len(), 5);
    let r1_name = res1_value["name"].as_str().unwrap();
    let r1_uid = res1_value["uid"].as_str().unwrap();
    let r1_created_at = res1_value["createdAt"].as_str().unwrap();
    let r1_updated_at = res1_value["updatedAt"].as_str().unwrap();
    assert_eq!(r1_name, "movies");
    assert_eq!(r1_uid.len(), 8);
    assert!(r1_created_at.len() > 1);
    assert!(r1_updated_at.len() > 1);

    // 2 - Check the list of indexes

    let (res2_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res2_value.as_array().unwrap().len(), 1);
    assert_eq!(res2_value[0].as_object().unwrap().len(), 5);
    let r2_0_name = res2_value[0]["name"].as_str().unwrap();
    let r2_0_uid = res2_value[0]["uid"].as_str().unwrap();
    let r2_0_created_at = res2_value[0]["createdAt"].as_str().unwrap();
    let r2_0_updated_at = res2_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(r2_0_name, r1_name);
    assert_eq!(r2_0_uid.len(), r1_uid.len());
    assert_eq!(r2_0_created_at.len(), r1_created_at.len());
    assert_eq!(r2_0_updated_at.len(), r1_updated_at.len());

    // 3 - Create a new index

    let body = json!({
        "name": "films",
    });

    let (res3_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 201);
    assert_eq!(res3_value.as_object().unwrap().len(), 5);
    let r3_name = res3_value["name"].as_str().unwrap();
    let r3_uid = res3_value["uid"].as_str().unwrap();
    let r3_created_at = res3_value["createdAt"].as_str().unwrap();
    let r3_updated_at = res3_value["updatedAt"].as_str().unwrap();
    assert_eq!(r3_name, "films");
    assert_eq!(r3_uid.len(), 8);
    assert!(r3_created_at.len() > 1);
    assert!(r3_updated_at.len() > 1);

    // 4 - Check the list of indexes

    let (res4_value, status_code) = server.list_indexes().await;

    assert_eq!(status_code, 200);
    assert_eq!(res4_value.as_array().unwrap().len(), 2);
    assert_eq!(res4_value[0].as_object().unwrap().len(), 5);
    let r4_0_name = res4_value[0]["name"].as_str().unwrap();
    let r4_0_uid = res4_value[0]["uid"].as_str().unwrap();
    let r4_0_created_at = res4_value[0]["createdAt"].as_str().unwrap();
    let r4_0_updated_at = res4_value[0]["updatedAt"].as_str().unwrap();
    assert_eq!(res4_value[1].as_object().unwrap().len(), 5);
    let r4_1_name = res4_value[1]["name"].as_str().unwrap();
    let r4_1_uid = res4_value[1]["uid"].as_str().unwrap();
    let r4_1_created_at = res4_value[1]["createdAt"].as_str().unwrap();
    let r4_1_updated_at = res4_value[1]["updatedAt"].as_str().unwrap();
    if r4_0_name == r1_name {
        assert_eq!(r4_0_name, r1_name);
        assert_eq!(r4_0_uid.len(), r1_uid.len());
        assert_eq!(r4_0_created_at.len(), r1_created_at.len());
        assert_eq!(r4_0_updated_at.len(), r1_updated_at.len());
    } else {
        assert_eq!(r4_0_name, r3_name);
        assert_eq!(r4_0_uid.len(), r3_uid.len());
        assert_eq!(r4_0_created_at.len(), r3_created_at.len());
        assert_eq!(r4_0_updated_at.len(), r3_updated_at.len());
    }
    if r4_1_name == r1_name {
        assert_eq!(r4_1_name, r1_name);
        assert_eq!(r4_1_uid.len(), r1_uid.len());
        assert_eq!(r4_1_created_at.len(), r1_created_at.len());
        assert_eq!(r4_1_updated_at.len(), r1_updated_at.len());
    } else {
        assert_eq!(r4_1_name, r3_name);
        assert_eq!(r4_1_uid.len(), r3_uid.len());
        assert_eq!(r4_1_created_at.len(), r3_created_at.len());
        assert_eq!(r4_1_updated_at.len(), r3_updated_at.len());
    }
}

#[actix_rt::test]
async fn create_index_failed() {
    let mut server = common::Server::with_uid("movies");

    // 2 - Push index creation with empty json body

    let body = json!({});

    let (res_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
    let message = res_value["message"].as_str().unwrap();
    assert_eq!(res_value.as_object().unwrap().len(), 4);
    assert_eq!(message, "Index creation must have an uid");

    // 3 - Create a index with extra data

    let body = json!({
        "name": "movies",
        "active": true
    });

    let (_res_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);

    // 3 - Create a index with wrong data type

    let body = json!({
        "name": "movies",
        "uid": 0
    });

    let (_res_value, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
}

// Resolve issue https://github.com/meilisearch/MeiliSearch/issues/492
#[actix_rt::test]
async fn create_index_with_primary_key_and_index() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index

    let body = json!({
        "uid": "movies",
        "primaryKey": "id",
    });

    let (_response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);

    // 2 - Add content

    let body = json!([{
        "id": 123,
        "text": "The mask"
    }]);

    server.add_or_replace_multiple_documents(body.clone()).await;

    // 3 - Retreive document

    let (response, _status_code) = server.get_document(123).await;

    let expect = json!({
        "id": 123,
        "text": "The mask"
    });

    assert_json_eq!(response, expect, ordered: false);
}

// Resolve issue https://github.com/meilisearch/MeiliSearch/issues/497
// Test when the given index uid is not valid
// Should have a 400 status code
// Should have the right error message
#[actix_rt::test]
async fn create_index_with_invalid_uid() {
    let mut server = common::Server::with_uid("");

    // 1 - Create the index with invalid uid

    let body = json!({
        "uid": "the movies"
    });

    let (response, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 4);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");

    // 2 - Create the index with invalid uid

    let body = json!({
        "uid": "%$#"
    });

    let (response, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 4);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");

    // 3 - Create the index with invalid uid

    let body = json!({
        "uid": "the~movies"
    });

    let (response, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 4);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");

    // 4 - Create the index with invalid uid

    let body = json!({
        "uid": "🎉"
    });

    let (response, status_code) = server.create_index(body).await;

    assert_eq!(status_code, 400);
    let message = response["message"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 4);
    assert_eq!(message, "Index must have a valid uid; Index uid can be of type integer or string only composed of alphanumeric characters, hyphens (-) and underscores (_).");
}

// Test that it's possible to add primary_key if it's not already set on index creation
#[actix_rt::test]
async fn create_index_and_add_indentifier_after() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index with no primary_key

    let body = json!({
        "uid": "movies",
    });
    let (response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"], json!(null));

    // 2 - Update the index and add an primary_key.

    let body = json!({
        "primaryKey": "id",
    });

    let (response, status_code) = server.update_index(body).await;
    assert_eq!(status_code, 200);
    eprintln!("response: {:#?}", response);
    assert_eq!(response["primaryKey"].as_str().unwrap(), "id");

    // 3 - Get index to verify if the primary_key is good

    let (response, status_code) = server.get_index().await;
    assert_eq!(status_code, 200);
    assert_eq!(response["primaryKey"].as_str().unwrap(), "id");
}

// Test that it's impossible to change the primary_key
#[actix_rt::test]
async fn create_index_and_update_indentifier_after() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index with no primary_key

    let body = json!({
        "uid": "movies",
        "primaryKey": "id",
    });
    let (response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"].as_str().unwrap(), "id");

    // 2 - Update the index and add an primary_key.

    let body = json!({
        "primaryKey": "skuid",
    });

    let (_response, status_code) = server.update_index(body).await;
    assert_eq!(status_code, 400);

    // 3 - Get index to verify if the primary_key still the first one

    let (response, status_code) = server.get_index().await;
    assert_eq!(status_code, 200);
    assert_eq!(response["primaryKey"].as_str().unwrap(), "id");
}

// Test that schema inference work well
#[actix_rt::test]
async fn create_index_without_primary_key_and_add_document() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index with no primary_key

    let body = json!({
        "uid": "movies",
    });
    let (response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"], json!(null));

    // 2 - Add a document

    let body = json!([{
        "id": 123,
        "title": "I'm a legend",
    }]);

    server.add_or_update_multiple_documents(body).await;

    // 3 - Get index to verify if the primary_key is good

    let (response, status_code) = server.get_index().await;
    assert_eq!(status_code, 200);
    assert_eq!(response["primaryKey"].as_str().unwrap(), "id");
}

// Test search with no primary_key
#[actix_rt::test]
async fn create_index_without_primary_key_and_search() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index with no primary_key

    let body = json!({
        "uid": "movies",
    });
    let (response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"], json!(null));

    // 2 - Search

    let query = "q=captain&limit=3";

    let (response, status_code) = server.search_get(&query).await;
    assert_eq!(status_code, 200);
    assert_eq!(response["hits"].as_array().unwrap().len(), 0);
}

// Test the error message when we push an document update and impossibility to find primary key
// Test issue https://github.com/meilisearch/MeiliSearch/issues/517
#[actix_rt::test]
async fn check_add_documents_without_primary_key() {
    let mut server = common::Server::with_uid("movies");

    // 1 - Create the index with no primary_key

    let body = json!({
        "uid": "movies",
    });
    let (response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"], json!(null));

    // 2- Add document

    let body = json!([{
      "title": "Test",
      "comment": "comment test"
    }]);

    let (response, status_code) = server.add_or_replace_multiple_documents_sync(body).await;

    assert_eq!(response.as_object().unwrap().len(), 4);
    assert_eq!(response["errorCode"], "missing_primary_key");
    assert_eq!(status_code, 400);
}

#[actix_rt::test]
async fn check_first_update_should_bring_up_processed_status_after_first_docs_addition() {
    let mut server = common::Server::with_uid("movies");

    let body = json!({
        "uid": "movies",
    });

    // 1. Create Index
    let (response, status_code) = server.create_index(body).await;
    assert_eq!(status_code, 201);
    assert_eq!(response["primaryKey"], json!(null));

    let dataset = include_bytes!("./assets/test_set.json");

    let body: Value = serde_json::from_slice(dataset).unwrap();

    // 2. Index the documents from movies.json, present inside of assets directory
    server.add_or_replace_multiple_documents(body).await;

    // 3. Fetch the status of the indexing done above.
    let (response, status_code) = server.get_all_updates_status().await;

    // 4. Verify the fetch is successful and indexing status is 'processed'
    assert_eq!(status_code, 200);
    assert_eq!(response[0]["status"], "processed");
}

#[actix_rt::test]
async fn get_empty_index() {
    let mut server = common::Server::with_uid("test");
    let (response, _status) = server.list_indexes().await;
    assert!(response.as_array().unwrap().is_empty());
}

#[actix_rt::test]
async fn create_and_list_multiple_indices() {
    let mut server = common::Server::with_uid("test");
    for i in 0..10 {
        server
            .create_index(json!({ "uid": format!("test{}", i) }))
            .await;
    }
    let (response, _status) = server.list_indexes().await;
    assert_eq!(response.as_array().unwrap().len(), 10);
}

#[actix_rt::test]
async fn get_unexisting_index_is_error() {
    let mut server = common::Server::with_uid("test");
    let (response, status) = server.get_index().await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(response["errorCode"], "index_not_found");
    assert_eq!(response["errorType"], "invalid_request_error");
}

#[actix_rt::test]
async fn create_index_twice_is_error() {
    let mut server = common::Server::with_uid("test");
    server.create_index(json!({ "uid": "test" })).await;
    let (response, status) = server.create_index(json!({ "uid": "test" })).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(response["errorCode"], "index_already_exists");
    assert_eq!(response["errorType"], "invalid_request_error");
}

#[actix_rt::test]
async fn badly_formatted_index_name_is_error() {
    let mut server = common::Server::with_uid("$__test");
    let (response, status) = server.create_index(json!({ "uid": "$__test" })).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(response["errorCode"], "invalid_index_uid");
    assert_eq!(response["errorType"], "invalid_request_error");
}

#[actix_rt::test]
async fn correct_response_no_primary_key_index() {
    let mut server = common::Server::with_uid("test");
    let (response, _status) = server.create_index(json!({ "uid": "test" })).await;
    assert_eq!(response["primaryKey"], Value::Null);
}

#[actix_rt::test]
async fn correct_response_with_primary_key_index() {
    let mut server = common::Server::with_uid("test");
    let (response, _status) = server
        .create_index(json!({ "uid": "test", "primaryKey": "test" }))
        .await;
    assert_eq!(response["primaryKey"], "test");
}

#[actix_rt::test]
async fn udpate_unexisting_index_is_error() {
    let mut server = common::Server::with_uid("test");
    let (response, status) = server.update_index(json!({ "primaryKey": "foobar" })).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(response["errorCode"], "index_not_found");
    assert_eq!(response["errorType"], "invalid_request_error");
}

#[actix_rt::test]
async fn update_existing_primary_key_is_error() {
    let mut server = common::Server::with_uid("test");
    server
        .create_index(json!({ "uid": "test", "primaryKey": "key" }))
        .await;
    let (response, status) = server.update_index(json!({ "primaryKey": "test2" })).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(response["errorCode"], "primary_key_already_present");
    assert_eq!(response["errorType"], "invalid_request_error");
}

#[actix_rt::test]
async fn test_facets_distribution_attribute() {
    let mut server = common::Server::test_server().await;

    let (response, _status_code) = server.get_index_stats().await;

    let expected = json!({
        "isIndexing": false,
        "numberOfDocuments":77,
        "fieldsDistribution":{
            "age":77,
            "gender":77,
            "phone":77,
            "name":77,
            "registered":77,
            "latitude":77,
            "email":77,
            "tags":77,
            "longitude":77,
            "color":77,
            "address":77,
            "balance":77,
            "about":77,
            "picture":77,
        },
    });

    assert_json_eq!(expected, response, ordered: true);
}