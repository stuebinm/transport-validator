use std::collections::HashSet;
use crate::validators::issues::*;

pub fn validate(gtfs: &gtfs_structures::Gtfs) -> Vec<Issue> {
    let mut used_stops = HashSet::new();

    // A stop can be used for a stop time
    for trip in gtfs.trips.values() {
        for stop_time in &trip.stop_times {
            used_stops.insert(stop_time.stop.id.to_owned());
        }
    }

    // A stop can be the parent station
    for stop in gtfs.stops.values() {
        for parent in &stop.parent_station {
            if used_stops.contains(&stop.id) {
                used_stops.insert(parent.to_owned());
            }
        }
    }

    gtfs.stops
        .iter()
        .filter(|&(_, stop)| !used_stops.contains(&stop.id))
        .map(|(_, stop)| Issue {
            severity: Severity::Error,
            issue_type: IssueType::UnusedStop,
            object_id: stop.id.to_owned(),
            object_name: Some(stop.name.to_owned()),
            related_object_id: None,
        }).collect()
}

#[test]
fn test() {
    let gtfs = gtfs_structures::Gtfs::new("test_data/unused_stop").unwrap();
    let issues = validate(&gtfs);

    assert_eq!(1, issues.len());
    assert_eq!("BOGUS", issues[0].object_id);
}
