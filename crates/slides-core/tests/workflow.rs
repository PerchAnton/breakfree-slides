use breakfree_slides_core::{
    commands::Editor,
    model::*,
    office::{self, Launch},
    render::{self, Viewport},
    storage,
};
use std::{ffi::OsString, io::Cursor, path::PathBuf, sync::Arc};

fn png() -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        12,
        8,
        image::Rgba([203, 111, 98, 255]),
    ))
    .write_to(&mut out, image::ImageFormat::Png)
    .unwrap();
    out.into_inner()
}
fn text() -> Content {
    Content::Text {
        text: "Hello, Україно!\nEditable text".into(),
        font_size: 32.0,
        bold: false,
        color: [23, 30, 41, 255],
    }
}
fn bounds() -> Bounds {
    Bounds {
        x: 145.5,
        y: 211.25,
        w: 375.0,
        h: 180.0,
    }
}

#[test]
fn three_slide_round_trip_preserves_every_supported_property_and_resource() {
    let dir = tempfile::tempdir().unwrap();
    let image_path = dir.path().join("original.png");
    std::fs::write(&image_path, png()).unwrap();
    let mut e = Editor::default();
    let text_id = e.add_object(0, text(), bounds()).unwrap();
    let second = e.add_slide(0).unwrap();
    e.add_object(
        second,
        Content::Rectangle {
            fill: [215, 110, 98, 255],
        },
        bounds(),
    )
    .unwrap();
    let third = e.add_slide(second).unwrap();
    let image_id = e
        .add_image(third, &storage::read_image(&image_path).unwrap())
        .unwrap();
    e.transform(
        0,
        text_id,
        Bounds {
            x: 52.0,
            y: 41.0,
            w: 540.0,
            h: 145.0,
        },
    )
    .unwrap();
    e.transform(third, image_id, bounds()).unwrap();
    e.reorder_slide(third, 0).unwrap();
    let arranged = e.deck.clone();
    e.undo();
    assert_ne!(e.deck, arranged);
    e.redo();
    assert_eq!(e.deck, arranged);
    let path = dir.path().join("roundtrip.bfslide");
    storage::save(&e.deck, &path).unwrap();
    e.mark_saved();
    assert!(!e.dirty());
    std::fs::remove_file(image_path).unwrap();
    drop(e);
    let opened = storage::load(&path).unwrap();
    assert_eq!(opened, arranged);
    let scene_before = render::scene(&arranged, 0);
    assert_eq!(render::scene(&opened, 0), scene_before);
    let mut e = Editor::new(opened);
    e.transform(
        0,
        image_id,
        Bounds {
            x: 200.0,
            ..bounds()
        },
    )
    .unwrap();
    assert!(e.dirty());
    e.undo();
    assert_eq!(e.deck, arranged);
    assert!(!e.dirty());
}
#[test]
fn undo_redo_covers_slide_create_duplicate_remove_reorder_and_unique_ids() {
    let mut e = Editor::default();
    e.add_object(0, text(), bounds()).unwrap();
    let before = e.deck.clone();
    e.duplicate_slide(0).unwrap();
    assert_ne!(e.deck.slides[0].id, e.deck.slides[1].id);
    assert_ne!(
        e.deck.slides[0].objects[0].id,
        e.deck.slides[1].objects[0].id
    );
    e.reorder_slide(1, 0).unwrap();
    e.remove_slide(1).unwrap();
    e.undo();
    e.undo();
    e.undo();
    assert_eq!(e.deck, before);
    e.redo();
    e.redo();
    e.redo();
    e.deck.validate().unwrap();
}
#[test]
fn z_order_hit_testing_and_delete_are_reversible() {
    let mut e = Editor::default();
    let a = e.add_object(0, text(), bounds()).unwrap();
    let b = e
        .add_object(
            0,
            Content::Rectangle {
                fill: [1, 2, 3, 255],
            },
            bounds(),
        )
        .unwrap();
    assert_eq!(render::hit_test(&e.deck, 0, 150.0, 220.0), Some(b));
    e.stack(0, b, false).unwrap();
    assert_eq!(render::hit_test(&e.deck, 0, 150.0, 220.0), Some(a));
    e.remove_object(0, a).unwrap();
    e.undo();
    assert_eq!(e.deck.slides[0].objects.last().unwrap().id, a);
}
#[test]
fn failed_command_is_transactional_and_does_not_destroy_redo() {
    let mut e = Editor::default();
    let before = e.deck.clone();
    e.add_slide(0).unwrap();
    e.undo();
    assert!(e.can_redo());
    assert!(e.transform(0, 999, bounds()).is_err());
    assert_eq!(e.deck, before);
    assert!(e.can_redo());
}
#[test]
fn last_slide_cannot_be_deleted() {
    let mut e = Editor::default();
    let d = e.deck.clone();
    assert!(e.remove_slide(0).is_err());
    assert_eq!(e.deck, d);
}
#[test]
fn text_notes_and_geometry_undo_restore_saved_dirty_state() {
    let mut e = Editor::default();
    let id = e.add_object(0, text(), bounds()).unwrap();
    e.mark_saved();
    let before = e.deck.clone();
    e.set_text(0, id, "Changed".into()).unwrap();
    e.apply(|d| {
        d.slides[0].notes = "Speaker notes".into();
        Ok(())
    })
    .unwrap();
    assert!(e.dirty());
    e.undo();
    e.undo();
    assert!(!e.dirty());
    assert_eq!(e.deck, before);
}
#[test]
fn duplicate_slides_share_immutable_images_and_keep_embedded_pixels() {
    let mut e = Editor::default();
    e.add_image(0, &png()).unwrap();
    e.duplicate_slide(0).unwrap();
    assert_eq!(e.deck.assets.len(), 1);
    let snapshot = e.deck.clone();
    let id = *snapshot.assets.keys().next().unwrap();
    assert!(Arc::ptr_eq(&snapshot.assets[&id], &e.deck.assets[&id]));
}
#[test]
fn damaged_foreign_and_future_files_are_rejected_without_source_changes() {
    let dir = tempfile::tempdir().unwrap();
    for (name, bytes) in [
        ("damaged", b"not json".to_vec()),
        (
            "foreign",
            serde_json::to_vec(&serde_json::json!({"kind":"other.app","content_version":1}))
                .unwrap(),
        ),
        ("future", {
            let mut d = Deck::new();
            d.content_version = 99;
            serde_json::to_vec(&d).unwrap()
        }),
    ] {
        let path = dir.path().join(name);
        std::fs::write(&path, &bytes).unwrap();
        assert!(storage::load(&path).is_err());
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }
}
#[test]
fn failed_save_preserves_previous_valid_document() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("safe.bfslide");
    let d = Deck::new();
    storage::save(&d, &path).unwrap();
    let original = std::fs::read(&path).unwrap();
    let mut invalid = d.clone();
    invalid.width = f64::NAN;
    assert!(storage::save(&invalid, &path).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), original);
    assert!(storage::save(&d, &dir.path().join("missing/child.bfslide")).is_err());
    assert_eq!(storage::load(&path).unwrap(), d);
}
#[test]
fn successful_atomic_replacement_can_be_reopened() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("replace.bfslide");
    let mut d = Deck::new();
    storage::save(&d, &path).unwrap();
    d.title = "Changed title".into();
    storage::save(&d, &path).unwrap();
    assert_eq!(storage::load(&path).unwrap(), d);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}
#[test]
fn replacement_failure_preserves_existing_target() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("existing");
    std::fs::create_dir(&target).unwrap();
    let sentinel = target.join("sentinel");
    std::fs::write(&sentinel, b"keep").unwrap();
    assert!(storage::save(&Deck::new(), &target).is_err());
    assert_eq!(std::fs::read(sentinel).unwrap(), b"keep");
}
#[test]
fn invalid_images_and_mismatched_metadata_are_rejected() {
    let mut e = Editor::default();
    let original = e.deck.clone();
    assert!(e.add_image(0, b"not a raster image").is_err());
    assert_eq!(e.deck, original);
    let mut d = Deck::new();
    let mut a = Asset::from_bytes(&png()).unwrap();
    a.width += 1;
    d.assets.insert(2, Arc::new(a));
    d.next_id = 3;
    assert!(d.validate().is_err());
}
#[test]
fn resource_missing_and_duplicate_ids_are_rejected() {
    let mut d = Deck::new();
    d.slides[0].objects.push(Object {
        id: 1,
        bounds: bounds(),
        content: Content::Image { asset: 77 },
    });
    assert!(d.validate().is_err());
    d.slides[0].objects[0].id = 2;
    d.next_id = 3;
    assert!(d.validate().is_err());
}
#[test]
fn image_and_document_size_limits_are_enforced() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("large.png");
    std::fs::write(&p, vec![0; MAX_IMAGE_BYTES + 1]).unwrap();
    assert!(storage::read_image(&p).is_err());
    let p = dir.path().join("large.bfslide");
    let f = std::fs::File::create(&p).unwrap();
    f.set_len(MAX_FILE_BYTES + 1).unwrap();
    assert!(storage::load(&p).is_err());
}
#[test]
fn unknown_fields_and_invalid_geometry_are_rejected() {
    let mut v = serde_json::to_value(Deck::new()).unwrap();
    v["unexpected"] = true.into();
    assert!(storage::decode(&serde_json::to_vec(&v).unwrap()).is_err());
    let mut e = Editor::default();
    assert!(
        e.add_object(
            0,
            text(),
            Bounds {
                w: -2.0,
                ..bounds()
            }
        )
        .is_err()
    );
    assert!(!e.can_undo());
}
#[test]
fn dpi_view_transforms_round_trip_without_mutating_deck() {
    let d = Deck::new();
    let original = d.clone();
    for dpi in [1.0, 1.25, 1.5, 2.0] {
        for zoom in [0.1, 0.75, 1.0, 2.0] {
            let view = Viewport {
                x: 18.0 * dpi,
                y: 42.0 * dpi,
                scale: dpi * zoom,
            };
            let b = view.project(bounds());
            let (x, y) = view.unproject(b.x, b.y);
            assert!((x - bounds().x).abs() < 1e-9);
            assert!((y - bounds().y).abs() < 1e-9);
        }
    }
    assert_eq!(d, original);
}
#[test]
fn renderer_scene_is_read_only_and_preserves_stack_and_aspect() {
    let mut e = Editor::default();
    e.add_object(0, text(), bounds()).unwrap();
    e.add_object(
        0,
        Content::Rectangle {
            fill: [1, 2, 3, 255],
        },
        bounds(),
    )
    .unwrap();
    let d = e.deck.clone();
    let s = render::scene(&d, 0);
    assert_eq!(s.len(), 2);
    for (w, h) in [(1440.0, 900.0), (800.0, 560.0), (1920.0, 1080.0)] {
        let v = Viewport::fit(d.width, d.height, w, h);
        assert!((d.width * v.scale / (d.height * v.scale) - 16.0 / 9.0).abs() < 1e-9);
    }
    assert_eq!(d, e.deck);
}
#[test]
fn office_contract_reports_real_build_identity_and_native_format() {
    let v: serde_json::Value =
        serde_json::from_str(&office::capabilities(env!("CARGO_PKG_VERSION"))).unwrap();
    assert_eq!(v["protocol_version"], 1);
    assert_eq!(v["app_id"], "breakfree.slides");
    assert_eq!(v["app_version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(v["formats"][0]["extensions"][0], "bfslide");
    assert_eq!(v["formats"][0]["id"], NATIVE_KIND);
    assert_eq!(v["formats"][0]["write"], true);
    assert!(!v.to_string().contains("pptx"));
}
#[test]
fn office_launch_requires_absolute_paths_and_rejects_ambiguous_flags() {
    let parse = |a: &[&str]| office::parse(a.iter().map(OsString::from));
    assert_eq!(parse(&["--new"]).unwrap(), Launch::New);
    assert_eq!(parse(&["--capabilities"]).unwrap(), Launch::Capabilities);
    let absolute = std::env::temp_dir().join("deck with spaces.bfslide");
    assert_eq!(
        office::parse([OsString::from("--open"), absolute.clone().into_os_string()]).unwrap(),
        Launch::Open(absolute)
    );
    assert!(parse(&["--open", "relative.bfslide"]).is_err());
    assert!(parse(&["--new", "--capabilities"]).is_err());
    assert!(parse(&["--unknown"]).is_err());
    assert!(parse(&["--open"]).is_err());
    assert!(parse(&["--ui-smoke", "relative"]).is_err());
    let _ = PathBuf::new();
}
