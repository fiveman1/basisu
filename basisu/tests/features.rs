//! With the optional source features disabled, UASTC LDR still transcodes and
//! the disabled sources are reported unsupported instead of decoding.
//! Run with `cargo test -p basisu --no-default-features --features std,zstd`.

#![cfg(all(
    not(target_arch = "wasm32"),
    not(feature = "etc1s"),
    not(feature = "astc-ldr"),
    not(feature = "hdr"),
    not(feature = "pvrtc")
))]

use basisu::{DecodeFlags, Error, SourceFormat, TargetFormat, Transcoder};

const UASTC: &[u8] = include_bytes!("fixtures/uastc.ktx2");
const ETC1S: &[u8] = include_bytes!("fixtures/etc1s.ktx2");
const ASTC_LDR: &[u8] = include_bytes!("../../corpus/smoke/astc_4x4_unorm.ktx2");
const UASTC_HDR: &[u8] = include_bytes!("../../corpus/smoke/uastc_hdr.ktx2");

#[test]
fn uastc_ldr_transcodes() {
    let t = Transcoder::new(UASTC).unwrap();
    assert_eq!(t.source_format(), SourceFormat::UastcLdr);
    for target in [
        TargetFormat::Bc7Rgba,
        TargetFormat::Bc5Rg,
        TargetFormat::Astc4x4Rgba,
        TargetFormat::EacRg11,
        TargetFormat::Rgba32,
    ] {
        assert!(t.supports(target), "{target:?}");
        t.transcode(0, target, DecodeFlags::NONE).unwrap();
    }
    assert!(!t.supports(TargetFormat::Pvrtc1_4Rgb));
}

#[test]
fn disabled_sources_are_unsupported() {
    for data in [ETC1S, ASTC_LDR, UASTC_HDR] {
        let t = Transcoder::new(data).unwrap();
        let target = if t.source_format() == SourceFormat::UastcHdr4x4 {
            TargetFormat::RgbaHalf
        } else {
            TargetFormat::Rgba32
        };
        assert!(!t.supports(target), "{:?}", t.source_format());
        assert!(matches!(
            t.transcode(0, target, DecodeFlags::NONE),
            Err(Error::Unsupported { .. })
        ));
    }
}
