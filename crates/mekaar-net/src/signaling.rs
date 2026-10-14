//! # mekaar-net: WebRTC Signaling Helpers
//!
//! Abstraksi pembuat dan parser paket sinyal WebRTC SDP Offer/Answer dan ICE Candidate
//! untuk panggilan audio/video darurat SOS via relay server.

use mekaar_proto::ClientCommand;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data representasi sinyal ICE Candidate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IceCandidatePayload {
    pub candidate: String,
    pub sdp_mid: Option<String>,
    pub sdp_mline_index: Option<u32>,
}

/// Data representasi sinyal SDP (Offer / Answer).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SdpPayload {
    pub sdp: String,
    pub sdp_type: String,
}

/// Helper untuk membuat perintah sinyal WebRTC SDP Offer.
pub fn make_offer_signal(target_user_id: Uuid, sdp: &str) -> ClientCommand {
    let payload = serde_json::to_string(&SdpPayload {
        sdp: sdp.to_string(),
        sdp_type: "offer".to_string(),
    })
    .unwrap_or_default();

    ClientCommand::WebRtcSignal {
        target_user_id,
        signal_type: "offer".to_string(),
        payload,
    }
}

/// Helper untuk membuat perintah sinyal WebRTC SDP Answer.
pub fn make_answer_signal(target_user_id: Uuid, sdp: &str) -> ClientCommand {
    let payload = serde_json::to_string(&SdpPayload {
        sdp: sdp.to_string(),
        sdp_type: "answer".to_string(),
    })
    .unwrap_or_default();

    ClientCommand::WebRtcSignal {
        target_user_id,
        signal_type: "answer".to_string(),
        payload,
    }
}

/// Helper untuk membuat perintah sinyal WebRTC ICE Candidate.
pub fn make_candidate_signal(
    target_user_id: Uuid,
    candidate: &str,
    sdp_mid: Option<String>,
    sdp_mline_index: Option<u32>,
) -> ClientCommand {
    let payload = serde_json::to_string(&IceCandidatePayload {
        candidate: candidate.to_string(),
        sdp_mid,
        sdp_mline_index,
    })
    .unwrap_or_default();

    ClientCommand::WebRtcSignal {
        target_user_id,
        signal_type: "candidate".to_string(),
        payload,
    }
}

/// Helper untuk membuat perintah sinyal panggilan ditutup (hangup).
pub fn make_hangup_signal(target_user_id: Uuid) -> ClientCommand {
    ClientCommand::WebRtcSignal {
        target_user_id,
        signal_type: "hangup".to_string(),
        payload: "{}".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_signaling_commands() {
        let target = Uuid::new_v4();
        let offer = make_offer_signal(target, "v=0\r\no=alice...");
        if let ClientCommand::WebRtcSignal {
            target_user_id,
            signal_type,
            payload,
        } = offer
        {
            assert_eq!(target_user_id, target);
            assert_eq!(signal_type, "offer");
            let sdp: SdpPayload = serde_json::from_str(&payload).unwrap();
            assert_eq!(sdp.sdp_type, "offer");
            assert!(sdp.sdp.contains("v=0"));
        } else {
            panic!("Format perintah bukan WebRtcSignal");
        }

        let cand = make_candidate_signal(target, "candidate:1 1 UDP ...", Some("0".to_string()), Some(0));
        if let ClientCommand::WebRtcSignal {
            signal_type,
            payload,
            ..
        } = cand
        {
            assert_eq!(signal_type, "candidate");
            let p: IceCandidatePayload = serde_json::from_str(&payload).unwrap();
            assert_eq!(p.sdp_mid, Some("0".to_string()));
        }
    }
}
