use crate::*;
use crate::{
    features::did::{DidDocument, DidMethod},
    tests::v3_setup::TestContextV3,
    types::*,
};

#[tokio::test]
async fn test_create_did_document() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-create-{}", uuid::Uuid::new_v4());
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };

    assert_eq!(status, 200, "Create DID response: {:?}", body);
    assert_eq!(body["did"], did);
    assert_eq!(body["document"]["did"], did);
    assert_eq!(body["document"]["is_active"], true);

    // Cleanup
    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_create_did_requires_auth() {
    let TestContextV3 { app, ddb, .. } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-auth-{}", uuid::Uuid::new_v4());
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }]
    });

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/did",
        body: {
            "did": &did,
            "document": did_document
        }
    };

    assert_eq!(status, 401, "Should require authentication");
}

#[tokio::test]
async fn test_create_did_validates_document() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-validate-{}", uuid::Uuid::new_v4());
    let invalid_document = serde_json::json!({
        "id": &did
    });

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": invalid_document
        }
    };

    assert_eq!(status, 422, "Should reject invalid document: {:?}", body);
}

#[tokio::test]
async fn test_create_did_validates_id_match() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-id-match-{}", uuid::Uuid::new_v4());
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": "did:web:different.com",
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };

    assert_eq!(status, 400, "Should reject mismatched DID: {:?}", body);
}

#[tokio::test]
async fn test_resolve_did() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-resolve-{}", uuid::Uuid::new_v4());
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };
    assert_eq!(status, 200);

    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200);
    assert_eq!(body["did"], did);

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_resolve_nonexistent_did() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let did = format!("did:web:nonexistent.example.com:{}", uuid::Uuid::new_v4());
    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_update_did() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-update-{}", uuid::Uuid::new_v4());
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };
    assert_eq!(status, 200);

    let updated_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-2", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkpTHR8VNsBxYAAWHut2Geadd9jSwuBV8xRoAnwWsdvktH"
        }],
        "authentication": [format!("{}#key-2", &did)]
    });

    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: test_user.1.clone(),
        body: { "document": updated_document }
    };

    assert_eq!(status, 200, "Update DID response: {:?}", body);
    assert_eq!(
        body["document"]["verificationMethod"][0]["id"],
        format!("{}#key-2", &did)
    );

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_update_did_enforces_ownership() {
    let TestContextV3 {
        app,
        test_user,
        user2,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!(
        "did:web:example.com:test-ownership-update-{}",
        uuid::Uuid::new_v4()
    );
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };
    assert_eq!(status, 200);

    let updated_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-2", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkpTHR8VNsBxYAAWHut2Geadd9jSwuBV8xRoAnwWsdvktH"
        }],
        "authentication": [format!("{}#key-2", &did)]
    });

    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: user2.1.clone(),
        body: { "document": updated_document }
    };

    assert_eq!(status, 401, "Should enforce ownership: {:?}", body);

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_deactivate_did() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!(
        "did:web:example.com:test-deactivate-{}",
        uuid::Uuid::new_v4()
    );
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };
    assert_eq!(status, 200);

    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200, "Deactivate DID response: {:?}", body);
    assert_eq!(
        body["message"].as_str().unwrap(),
        "DID successfully deactivated"
    );

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_deactivate_did_enforces_ownership() {
    let TestContextV3 {
        app,
        test_user,
        user2,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!(
        "did:web:example.com:test-ownership-deactivate-{}",
        uuid::Uuid::new_v4()
    );
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, _body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };
    assert_eq!(status, 200);

    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, body) = delete! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: user2.1.clone()
    };

    assert_eq!(status, 401, "Should enforce ownership: {:?}", body);

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_create_did_with_multiple_verification_methods() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!(
        "did:web:example.com:test-multi-keys-{}",
        uuid::Uuid::new_v4()
    );
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [
            {
                "id": format!("{}#key-1", &did),
                "type": "Ed25519VerificationKey2020",
                "controller": &did,
                "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
            },
            {
                "id": format!("{}#key-2", &did),
                "type": "X25519KeyAgreementKey2020",
                "controller": &did,
                "publicKeyMultibase": "z6LSbysY2xFMRpGMhb7tFTLMpeuPRaqaWM1yECx2AtzE3KCc"
            }
        ],
        "authentication": [format!("{}#key-1", &did)],
        "keyAgreement": [format!("{}#key-2", &did)]
    });

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };

    assert_eq!(
        status, 200,
        "Create DID with multiple keys response: {:?}",
        body
    );
    assert_eq!(
        body["document"]["document"]["verificationMethod"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_create_did_with_services() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!("did:web:example.com:test-services-{}", uuid::Uuid::new_v4());
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "service": [
            {
                "id": format!("{}#messaging", &did),
                "type": "MessagingService",
                "serviceEndpoint": "https://example.com/messaging"
            },
            {
                "id": format!("{}#linked-domain", &did),
                "type": "LinkedDomains",
                "serviceEndpoint": "https://example.com"
            }
        ]
    });

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };

    assert_eq!(status, 200, "Create DID with services response: {:?}", body);
    assert_eq!(
        body["document"]["document"]["service"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}

#[tokio::test]
async fn test_did_url_encoding() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let did = format!(
        "did:web:example.com:3000:test-encoding-{}",
        uuid::Uuid::new_v4()
    );
    let did_document = serde_json::json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": &did,
        "verificationMethod": [{
            "id": format!("{}#key-1", &did),
            "type": "Ed25519VerificationKey2020",
            "controller": &did,
            "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
        }],
        "authentication": [format!("{}#key-1", &did)]
    });

    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/did",
        headers: test_user.1.clone(),
        body: {
            "did": &did,
            "document": did_document
        }
    };

    assert_eq!(
        status, 200,
        "Create DID with encoded characters: {:?}",
        body
    );

    let encoded_did =
        percent_encoding::utf8_percent_encode(&did, percent_encoding::NON_ALPHANUMERIC).to_string();
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/did/{}", encoded_did),
        headers: test_user.1.clone()
    };

    assert_eq!(status, 200, "Resolve encoded DID response: {:?}", body);
    assert_eq!(body["did"], did);

    let _ = crate::features::did::StoredDidDocument::delete(
        &ddb,
        &Partition::Did(did.to_string()),
        Some(&EntityType::DidDocument),
    )
    .await;
}
