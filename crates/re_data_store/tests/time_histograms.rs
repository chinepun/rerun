use std::collections::BTreeSet;

use re_arrow_store::GarbageCollectionOptions;
use re_data_store::StoreDb;
use re_int_histogram::RangeI64;
use re_log_types::{
    example_components::{MyColor, MyPoint},
    DataRow, EntityPath, RowId, StoreId, TimeInt, TimePoint, Timeline,
};
use re_types_core::{
    components::{ClearIsRecursive, InstanceKey},
    ComponentName, Loggable,
};

// TODO(cmc): This should also test for the timeless counts but right now they're a bit all over
// the place, so need to land new reworked EntityTree etc first.

// ---

// Complete test suite for our various time histograms which, among other things, power the
// timeline widget.
#[test]
fn time_histograms() -> anyhow::Result<()> {
    let mut db = StoreDb::new(StoreId::random(re_log_types::StoreKind::Recording));

    let timeline_frame = Timeline::new_sequence("frame");
    let timeline_other = Timeline::new_temporal("other");
    let timeline_yet_another = Timeline::new_sequence("yet_another");

    let entity_parent: EntityPath = "parent".into();
    let entity_grandchild: EntityPath = "parent/child/grandchild".into();
    let entity_unrelated: EntityPath = "very/unrelated".into();

    // Single top-level entity, explicitly logged `InstanceKey`s.
    {
        let row = DataRow::from_component_batches(
            RowId::new(),
            TimePoint::from_iter([
                (timeline_frame, 42.into()),      //
                (timeline_other, 666.into()),     //
                (timeline_yet_another, 1.into()), //
            ]),
            entity_parent.clone(),
            [&InstanceKey::from_iter(0..10) as _],
        )?;

        db.add_data_row(row)?;

        // times per timeline
        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[42])),
                (&timeline_other, Some(&[666])),
                (&timeline_yet_another, Some(&[1])),
            ],
        );

        // histograms per timeline
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline ('parent')
        assert_histogram_for_component(
            &db,
            &entity_parent,
            InstanceKey::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 3],
        );
    }

    // Grand-child, multiple components, auto-generated `InstanceKey`s.
    {
        let row = {
            let num_instances = 3;
            let points: Vec<_> = (0..num_instances)
                .map(|i| MyPoint::new(0.0, i as f32))
                .collect();
            let colors = vec![MyColor::from(0xFF0000FF)];
            DataRow::from_component_batches(
                RowId::new(),
                TimePoint::from_iter([
                    (timeline_frame, 42.into()),      //
                    (timeline_yet_another, 1.into()), //
                ]),
                entity_grandchild.clone(),
                [&points as _, &colors as _],
            )?
        };

        db.add_data_row(row.clone())?;

        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[42])),
                (&timeline_other, Some(&[666])),
                (&timeline_yet_another, Some(&[1])),
            ],
        );

        // histograms per timeline
        // NOTE: notice how autogenerated instance keys are ignored.
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 3)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 3)])),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline ('parent')
        assert_histogram_for_component(
            &db,
            &entity_parent,
            InstanceKey::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 3],
        );
        // NOTE: per-component histograms are not recursive!
        assert_histogram_for_component(
            &db,
            &entity_parent,
            MyPoint::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_parent,
            MyColor::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );

        // histograms per component per timeline ('parent/child/grandchild')
        // NOTE: notice how autogenerated instance keys are ignored!
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [(&timeline_frame, None), (&timeline_yet_another, None)] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyPoint::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyColor::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 2],
        );

        db.add_data_row(row)?; // same row a second time!

        // times per timeline
        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[42])),
                (&timeline_other, Some(&[666])),
                (&timeline_yet_another, Some(&[1])),
            ],
        );

        // histograms per timeline
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 5)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 5)])),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline ('parent/child/grandchild')
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [(&timeline_frame, None), (&timeline_yet_another, None)] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyPoint::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 2)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 2)])),
            ] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyColor::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 2)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 2)])),
            ] as [(_, Option<&[_]>); 2],
        );
    }

    // Grand-child, timeless additions.
    {
        let row = {
            let num_instances = 6;
            let colors = vec![MyColor::from(0x00DD00FF); num_instances];
            DataRow::from_component_batches(
                RowId::new(),
                TimePoint::timeless(),
                "entity".into(),
                [
                    &InstanceKey::from_iter(0..num_instances as _) as _,
                    &colors as _,
                ],
            )?
        };

        db.add_data_row(row)?;

        // times per timeline
        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[42])),
                (&timeline_other, Some(&[666])),
                (&timeline_yet_another, Some(&[1])),
            ],
        );

        // histograms per timeline
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 5)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 5)])),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline
        assert_histogram_for_component(
            &db,
            &entity_parent,
            MyPoint::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_parent,
            MyColor::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [(&timeline_frame, None), (&timeline_yet_another, None)] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyPoint::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 2)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 2)])),
            ] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyColor::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 2)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 2)])),
            ] as [(_, Option<&[_]>); 2],
        );
    }

    // Completely unrelated entity.
    {
        let row = {
            let num_instances = 3;
            let points: Vec<_> = (0..num_instances)
                .map(|i| MyPoint::new(0.0, i as f32))
                .collect();
            let colors = vec![MyColor::from(0xFF0000FF)];
            DataRow::from_component_batches(
                RowId::new(),
                TimePoint::from_iter([
                    (timeline_frame, 1234.into()),       //
                    (timeline_other, 1235.into()),       //
                    (timeline_yet_another, 1236.into()), //
                ]),
                entity_unrelated.clone(),
                [
                    &InstanceKey::from_iter(0..num_instances) as _,
                    &points as _,
                    &colors as _,
                ],
            )?
        };

        db.add_data_row(row)?;

        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[42, 1234])),
                (&timeline_other, Some(&[666, 1235])),
                (&timeline_yet_another, Some(&[1, 1236])),
            ],
        );

        // histograms per timeline
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (
                    &timeline_frame,
                    Some(&[(RangeI64::new(42, 42), 5), (RangeI64::new(1234, 1234), 3)]),
                ),
                (
                    &timeline_other,
                    Some(&[(RangeI64::new(666, 666), 1), (RangeI64::new(1235, 1235), 3)]),
                ),
                (
                    &timeline_yet_another,
                    Some(&[(RangeI64::new(1, 1), 5), (RangeI64::new(1236, 1236), 3)]),
                ),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline
        assert_histogram_for_component(
            &db,
            &entity_parent,
            InstanceKey::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(42, 42), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 3],
        );
        // NOTE: per-component histograms are not recursive!
        assert_histogram_for_component(
            &db,
            &entity_parent,
            MyPoint::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_parent,
            MyColor::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );

        // histograms per component per timeline ('very/unrelated')
        assert_histogram_for_component(
            &db,
            &entity_unrelated,
            InstanceKey::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(1234, 1234), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(1235, 1235), 1)])),
                (
                    &timeline_yet_another,
                    Some(&[(RangeI64::new(1236, 1236), 1)]),
                ),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_unrelated,
            MyPoint::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(1234, 1234), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(1235, 1235), 1)])),
                (
                    &timeline_yet_another,
                    Some(&[(RangeI64::new(1236, 1236), 1)]),
                ),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_unrelated,
            MyColor::name(),
            [
                (&timeline_frame, Some(&[(RangeI64::new(1234, 1234), 1)])),
                (&timeline_other, Some(&[(RangeI64::new(1235, 1235), 1)])),
                (
                    &timeline_yet_another,
                    Some(&[(RangeI64::new(1236, 1236), 1)]),
                ),
            ] as [(_, Option<&[_]>); 3],
        );
    }

    // Immediate clear.
    {
        let row = {
            DataRow::from_component_batches(
                RowId::new(),
                TimePoint::from_iter([
                    (timeline_frame, 1000.into()), //
                ]),
                entity_parent.clone(),
                [&[ClearIsRecursive(true)] as _],
            )?
        };

        db.add_data_row(row)?;

        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[42, 1000, 1234])),
                (&timeline_other, Some(&[666, 1235])),
                (&timeline_yet_another, Some(&[1, 1236])),
            ],
        );

        // histograms per timeline
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (
                    &timeline_frame,
                    Some(&[
                        (RangeI64::new(42, 42), 5),
                        // We're clearing the parent's `InstanceKey` as well as the grandchild's
                        // `MyPoint`, `MyColor` and `InstanceKey`. That's four.
                        (RangeI64::new(1000, 1000), 4),
                        (RangeI64::new(1234, 1234), 3),
                    ]),
                ),
                (
                    &timeline_other,
                    Some(&[(RangeI64::new(666, 666), 1), (RangeI64::new(1235, 1235), 3)]),
                ),
                (
                    &timeline_yet_another,
                    Some(&[(RangeI64::new(1, 1), 5), (RangeI64::new(1236, 1236), 3)]),
                ),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline
        assert_histogram_for_component(
            &db,
            &entity_parent,
            InstanceKey::name(),
            [
                (
                    &timeline_frame,
                    Some(&[(RangeI64::new(42, 42), 1), (RangeI64::new(1000, 1000), 1)]),
                ),
                (&timeline_other, Some(&[(RangeI64::new(666, 666), 1)])),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 1)])),
            ] as [(_, Option<&[_]>); 3],
        );

        // histograms per component per timeline
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [(&timeline_frame, None), (&timeline_yet_another, None)] as [(_, Option<&[_]>); 2],
        );
        // NOTE: even though the component was logged twice at the same timestamp, the clear will
        // only inject once!
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyPoint::name(),
            [
                (
                    &timeline_frame,
                    Some(&[(RangeI64::new(42, 42), 2), (RangeI64::new(1000, 1000), 1)]),
                ),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 2)])),
            ] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyColor::name(),
            [
                (
                    &timeline_frame,
                    Some(&[(RangeI64::new(42, 42), 2), (RangeI64::new(1000, 1000), 1)]),
                ),
                (&timeline_yet_another, Some(&[(RangeI64::new(1, 1), 2)])),
            ] as [(_, Option<&[_]>); 2],
        );
    }

    // Full GC
    {
        db.gc(&GarbageCollectionOptions::gc_everything());

        assert_times_per_timeline(
            &db,
            [
                (&Timeline::log_time(), Some(&[] as &[i64])),
                (&timeline_frame, Some(&[])),
                (&timeline_other, Some(&[])),
                (&timeline_yet_another, Some(&[])),
            ],
        );

        // histograms per timeline
        assert_recursive_histogram(
            &db,
            [
                (&Timeline::log_time(), None),
                (&timeline_frame, Some(&[])),
                (&timeline_other, Some(&[])),
                (&timeline_yet_another, Some(&[])),
            ] as [(_, Option<&[_]>); 4],
        );

        // histograms per component per timeline
        assert_histogram_for_component(
            &db,
            &entity_parent,
            InstanceKey::name(),
            [
                (&timeline_frame, Some(&[])),
                (&timeline_other, Some(&[])),
                (&timeline_yet_another, Some(&[])),
            ] as [(_, Option<&[_]>); 3],
        );

        // histograms per component per timeline
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [
                (&timeline_frame, None),
                (&timeline_other, None),
                (&timeline_yet_another, None),
            ] as [(_, Option<&[_]>); 3],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            InstanceKey::name(),
            [(&timeline_frame, None), (&timeline_yet_another, None)] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyPoint::name(),
            [
                (&timeline_frame, Some(&[])),
                (&timeline_yet_another, Some(&[])),
            ] as [(_, Option<&[_]>); 2],
        );
        assert_histogram_for_component(
            &db,
            &entity_grandchild,
            MyColor::name(),
            [
                (&timeline_frame, Some(&[])),
                (&timeline_yet_another, Some(&[])),
            ] as [(_, Option<&[_]>); 2],
        );
    }

    Ok(())
}

/// Checks the state of the global time tracker (at the `EntityDb` level).
fn assert_times_per_timeline<'a>(
    db: &StoreDb,
    expected: impl IntoIterator<Item = (&'a Timeline, Option<&'a [impl Into<TimeInt> + Copy + 'a]>)>,
) {
    for (timeline, expected_times) in expected {
        let times = db.times_per_timeline().get(timeline);

        if let Some(expected) = expected_times {
            let times: BTreeSet<_> = times.unwrap().keys().copied().collect();
            let expected: BTreeSet<_> = expected.iter().map(|t| (*t).into()).collect();
            similar_asserts::assert_eq!(expected, times);
        } else {
            assert!(times.is_none());
        }
    }
}

/// Checks the state of the per-EntityTree recursive time tracker.
fn assert_recursive_histogram<'a>(
    db: &StoreDb,
    expected: impl IntoIterator<Item = (&'a Timeline, Option<&'a [(RangeI64, u64)]>)>,
) {
    for (timeline, expected_times) in expected {
        let histo = db.time_histogram(timeline);

        if let Some(expected) = expected_times {
            let histo = histo.unwrap();
            let ranges = histo.range(i64::MIN.., 0).collect::<Vec<_>>();
            let expected: Vec<_> = expected.to_vec();
            similar_asserts::assert_eq!(expected, ranges);
        } else {
            assert!(histo.is_none());
        }
    }
}

/// Checks the state of the per-`EntityTree` per-`ComponentName` flat time tracker.
fn assert_histogram_for_component<'a>(
    db: &StoreDb,
    entity_path: &EntityPath,
    component_name: ComponentName,
    expected: impl IntoIterator<Item = (&'a Timeline, Option<&'a [(RangeI64, u64)]>)>,
) {
    for (timeline, expected_times) in expected {
        let histo = db
            .tree()
            .subtree(entity_path)
            .and_then(|tree| tree.time_histogram_for_component(timeline, component_name));

        if let Some(expected) = expected_times {
            let histo = histo.unwrap();
            let ranges = histo.range(i64::MIN.., 0).collect::<Vec<_>>();
            let expected: Vec<_> = expected.to_vec();
            similar_asserts::assert_eq!(expected, ranges);
        } else {
            assert!(histo.is_none());
        }
    }
}
