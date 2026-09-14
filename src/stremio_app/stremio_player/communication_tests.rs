use crate::stremio_app::stremio_player::communication::{
    BoolProp, CmdVal, FpProp, InMsg, InMsgArgs, InMsgFn, IntProp, MpvCmd, PlayerEnded,
    PlayerProprChange, PlayerResponse, PropKey, PropVal,
};
use libmpv2::{events::PropertyData, mpv_end_file_reason};

use serde_test::{assert_tokens, Token};

#[test]
fn video_ready_response() {
    assert_eq!(
        PlayerResponse::video_ready(7, true).to_value(),
        Some(serde_json::json!([
            "mpv-event-video-ready",
            { "loadId": 7, "ready": true }
        ]))
    );
}

#[test]
fn propr_change_tokens() {
    let prop = "test-prop";
    let tokens: [Token; 6] = [
        Token::Struct {
            name: "PlayerProprChange",
            len: 2,
        },
        Token::Str("name"),
        Token::None,
        Token::Str("data"),
        Token::None,
        Token::StructEnd,
    ];

    fn tokens_by_type(tokens: &[Token; 6], name: &'static str, val: PropertyData, token: Token) {
        let mut typed_tokens = tokens.clone();
        typed_tokens[2] = Token::Str(name);
        typed_tokens[4] = token;
        assert_tokens(
            &PlayerProprChange::from_name_value(name.to_string(), val),
            &typed_tokens,
        );
    }
    tokens_by_type(&tokens, prop, PropertyData::Flag(true), Token::Bool(true));
    tokens_by_type(&tokens, prop, PropertyData::Int64(1), Token::F64(1.0));
    tokens_by_type(&tokens, prop, PropertyData::Double(1.0), Token::F64(1.0));
    tokens_by_type(&tokens, prop, PropertyData::OsdStr("ok"), Token::Str("ok"));
    tokens_by_type(&tokens, prop, PropertyData::Str("ok"), Token::Str("ok"));

    // JSON response
    tokens_by_type(
        &tokens,
        "track-list",
        PropertyData::Str(r#""ok""#),
        Token::Str("ok"),
    );
    tokens_by_type(
        &tokens,
        "video-params",
        PropertyData::Str(r#""ok""#),
        Token::Str("ok"),
    );
    tokens_by_type(
        &tokens,
        "metadata",
        PropertyData::Str(r#""ok""#),
        Token::Str("ok"),
    );
}

#[test]
fn ended_tokens() {
    let error_tokens: [Token; 12] = [
        Token::Struct {
            name: "PlayerEnded",
            len: 2,
        },
        Token::Str("reason"),
        Token::Str("error"),
        Token::Str("error"),
        Token::Some,
        Token::Struct {
            name: "PlayerEndedError",
            len: 2,
        },
        Token::Str("message"),
        Token::Str("Unknown error"),
        Token::Str("critical"),
        Token::Bool(false),
        Token::StructEnd,
        Token::StructEnd,
    ];
    let tokens: [Token; 4] = [
        Token::Struct {
            name: "PlayerEnded",
            len: 1,
        },
        Token::Str("reason"),
        Token::Str("quit"),
        Token::StructEnd,
    ];
    assert_tokens(
        &PlayerEnded::from_end_reason(mpv_end_file_reason::Error),
        &error_tokens,
    );
    assert_tokens(
        &PlayerEnded::from_end_reason(mpv_end_file_reason::Quit),
        &tokens,
    );
    let eof_tokens: [Token; 4] = [
        Token::Struct {
            name: "PlayerEnded",
            len: 1,
        },
        Token::Str("reason"),
        Token::Str("eof"),
        Token::StructEnd,
    ];
    assert_tokens(
        &PlayerEnded::from_end_reason(mpv_end_file_reason::Eof),
        &eof_tokens,
    );
    let stop_tokens: [Token; 4] = [
        Token::Struct {
            name: "PlayerEnded",
            len: 1,
        },
        Token::Str("reason"),
        Token::Str("stop"),
        Token::StructEnd,
    ];
    assert_tokens(
        &PlayerEnded::from_end_reason(mpv_end_file_reason::Stop),
        &stop_tokens,
    );
}

#[test]
fn ob_propr_tokens() {
    assert_tokens(
        &InMsg(
            InMsgFn::MpvObserveProp,
            InMsgArgs::ObProp(PropKey::Bool(BoolProp::Pause)),
        ),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-observe-prop"),
            Token::Str("pause"),
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn set_propr_tokens() {
    assert_tokens(
        &InMsg(
            InMsgFn::MpvSetProp,
            InMsgArgs::StProp(PropKey::Bool(BoolProp::Pause), PropVal::Bool(true)),
        ),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-set-prop"),
            Token::Tuple { len: 2 },
            Token::Str("pause"),
            Token::Bool(true),
            Token::TupleEnd,
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn set_secondary_sid_tokens() {
    assert_tokens(
        &InMsg(
            InMsgFn::MpvSetProp,
            InMsgArgs::StProp(PropKey::Int(IntProp::SecondarySid), PropVal::Num(7.0)),
        ),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-set-prop"),
            Token::Tuple { len: 2 },
            Token::Str("secondary-sid"),
            Token::F64(7.0),
            Token::TupleEnd,
            Token::TupleStructEnd,
        ],
    );

    let parsed: InMsg = serde_json::from_str(r#"["mpv-set-prop",["secondary-sid","no"]]"#)
        .expect("secondary-sid should deserialize from web UI IPC");
    assert_eq!(
        parsed,
        InMsg(
            InMsgFn::MpvSetProp,
            InMsgArgs::StProp(
                PropKey::Int(IntProp::SecondarySid),
                PropVal::Str("no".to_string()),
            ),
        )
    );
}

#[test]
fn set_secondary_sub_delay_tokens() {
    assert_tokens(
        &InMsg(
            InMsgFn::MpvSetProp,
            InMsgArgs::StProp(PropKey::Fp(FpProp::SecondarySubDelay), PropVal::Num(-5.0)),
        ),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-set-prop"),
            Token::Tuple { len: 2 },
            Token::Str("secondary-sub-delay"),
            Token::F64(-5.0),
            Token::TupleEnd,
            Token::TupleStructEnd,
        ],
    );

    let parsed: InMsg = serde_json::from_str(r#"["mpv-set-prop",["secondary-sub-delay",-4.5]]"#)
        .expect("secondary-sub-delay should deserialize from web UI IPC");
    assert_eq!(
        parsed,
        InMsg(
            InMsgFn::MpvSetProp,
            InMsgArgs::StProp(PropKey::Fp(FpProp::SecondarySubDelay), PropVal::Num(-4.5),),
        )
    );

    let observed: InMsg = serde_json::from_str(r#"["mpv-observe-prop","secondary-sub-delay"]"#)
        .expect("secondary-sub-delay should deserialize for observation");
    assert_eq!(
        observed,
        InMsg(
            InMsgFn::MpvObserveProp,
            InMsgArgs::ObProp(PropKey::Fp(FpProp::SecondarySubDelay)),
        )
    );
}

#[test]
fn set_gpu_video_processing_tokens() {
    assert_tokens(
        &InMsg(InMsgFn::MpvSetGpuVideoProcessing, InMsgArgs::Flag(true)),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-set-gpu-video-processing"),
            Token::Bool(true),
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn command_stop_tokens() {
    assert_tokens(
        &InMsg(
            InMsgFn::MpvCommand,
            InMsgArgs::Cmd(CmdVal::Single((MpvCmd::Stop,))),
        ),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-command"),
            Token::Tuple { len: 1 },
            Token::Str("stop"),
            Token::TupleEnd,
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn command_sub_add_tokens() {
    let command = InMsg(
        InMsgFn::MpvCommand,
        InMsgArgs::Cmd(CmdVal::Quintuple(
            MpvCmd::SubAdd,
            "https://example.com/subtitles.srt".to_string(),
            "auto".to_string(),
            "TwinCue secondary".to_string(),
            "eng".to_string(),
        )),
    );

    assert_tokens(
        &command,
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-command"),
            Token::Tuple { len: 5 },
            Token::Str("sub-add"),
            Token::Str("https://example.com/subtitles.srt"),
            Token::Str("auto"),
            Token::Str("TwinCue secondary"),
            Token::Str("eng"),
            Token::TupleEnd,
            Token::TupleStructEnd,
        ],
    );

    let parsed: InMsg = serde_json::from_str(
        r#"["mpv-command",["sub-add","https://example.com/subtitles.srt","auto","TwinCue secondary","eng"]]"#,
    )
    .expect("sub-add should deserialize from web UI IPC");
    assert_eq!(parsed, command);
}

#[test]
fn command_loadfile_tokens() {
    assert_tokens(
        &InMsg(
            InMsgFn::MpvCommand,
            InMsgArgs::Cmd(CmdVal::Double(MpvCmd::Loadfile, "some_file".to_string())),
        ),
        &[
            Token::TupleStruct {
                name: "InMsg",
                len: 2,
            },
            Token::Str("mpv-command"),
            Token::Tuple { len: 2 },
            Token::Str("loadfile"),
            Token::Str("some_file"),
            Token::TupleEnd,
            Token::TupleStructEnd,
        ],
    );
}
