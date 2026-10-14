//! Integration Tests: mekaar-proto Wire Protocol & Codecs

use mekaar_proto::*;
use uuid::Uuid;

#[test]
fn test_client_server_commands_roundtrip() {
    let user_id = Uuid::new_v4();
    let target_id = Uuid::new_v4();
    let room_id = Uuid::new_v4();

    // 1. ClientCommand: AuthSubscribe
    let auth_cmd = ClientCommand::AuthSubscribe {
        user_id,
        public_key: [0x42; 32],
    };
    let auth_bin = encode_postcard(&auth_cmd).expect("postcard encode auth");
    let auth_dec: ClientCommand = decode_postcard(&auth_bin).expect("postcard decode auth");
    assert_eq!(auth_cmd, auth_dec);

    // 2. ClientCommand: SendEnvelope
    let envelope = MessageEnvelope::new(
        user_id,
        target_id,
        room_id,
        [0xAA; 24],
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        1700000000000,
        MessageType::Text,
    )
    .with_view_once(true)
    .with_expiry(Some(1700003600000));

    let send_cmd = ClientCommand::SendEnvelope(envelope.clone());
    let send_bin = encode_postcard(&send_cmd).expect("postcard encode send");
    let send_dec: ClientCommand = decode_postcard(&send_bin).expect("postcard decode send");
    assert_eq!(send_cmd, send_dec);

    // 3. ClientCommand: SosTrigger
    let sos_cmd = ClientCommand::SosTrigger {
        user_id,
        latitude: -6.208821,
        longitude: 106.845599,
        accuracy: 3.5,
        timestamp_ms: 1700000001000,
    };
    let sos_bin = encode_postcard(&sos_cmd).expect("postcard encode sos");
    let sos_dec: ClientCommand = decode_postcard(&sos_bin).expect("postcard decode sos");
    assert_eq!(sos_cmd, sos_dec);

    // 4. ServerCommand: GuardianSosBroadcast
    let broadcast_cmd = ServerCommand::GuardianSosBroadcast {
        victim_id: user_id,
        latitude: -6.208821,
        longitude: 106.845599,
        accuracy: 3.5,
        timestamp_ms: 1700000001000,
    };
    let bcast_bin = encode_postcard(&broadcast_cmd).expect("postcard encode bcast");
    let bcast_dec: ServerCommand = decode_postcard(&bcast_bin).expect("postcard decode bcast");
    assert_eq!(broadcast_cmd, bcast_dec);
}

#[test]
fn test_postcard_vs_json_efficiency() {
    let envelope = MessageEnvelope::new(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        [0x12; 24],
        vec![0xFF; 256], // 256 bytes payload ciphertext
        1700000000000,
        MessageType::Location,
    )
    .with_view_once(false)
    .with_expiry(Some(1700086400000));

    let bin_postcard = encode_postcard(&envelope).expect("postcard encode");
    let str_json = encode_json(&envelope).expect("json encode");

    let postcard_size = bin_postcard.len();
    let json_size = str_json.as_bytes().len();

    // Verifikasi penghematan kuota: Postcard biner harus jauh lebih hemat dibanding JSON
    assert!(
        postcard_size < json_size,
        "Postcard size ({} bytes) harus lebih kecil dari JSON ({} bytes)",
        postcard_size,
        json_size
    );

    let reduction_pct = ((json_size as f64 - postcard_size as f64) / json_size as f64) * 100.0;
    println!(
        "Ukuran Postcard: {} bytes vs JSON: {} bytes (Hemat: {:.1}%)",
        postcard_size, json_size, reduction_pct
    );
    assert!(reduction_pct > 35.0, "Penghematan harus signifikan");
}

#[test]
fn test_corrupted_packet_fails() {
    let mut invalid_bytes = vec![0xFF, 0xFE, 0xFD];
    let result: Result<MessageEnvelope, _> = decode_postcard(&invalid_bytes);
    assert!(result.is_err());

    invalid_bytes.clear();
    let result_json: Result<MessageEnvelope, _> = decode_json("{corrupted_json");
    assert!(result_json.is_err());
}
