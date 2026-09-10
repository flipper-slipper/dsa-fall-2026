mod meeting;

use meeting::Meeting;

fn main() {
    let meetings = Meeting::get_meetings();
    println!("loaded {} meetings", meetings.len());
    println!("conflict (pairs):  {}", conflict_via_pairs(&meetings));
    println!("conflict (sorted): {}", sort_conflict(&meetings));
}


/// A function that checks for conflicts by simple pair checking
pub fn conflict_via_pairs(meetings: &[Meeting]) -> bool {
    for i in 0..meetings.len() {
        for j in (i + 1)..meetings.len() {
            let a = &meetings[i];
            let b = &meetings[j];
            if a.start < b.end && b.start < a.end {
                return true;
            }
        }
    }
    false
}

/// A function that checks for conflicts by first sorting
pub fn sort_conflict(meetings: &[Meeting]) -> bool {
    let mut sorted: Vec<&Meeting> = meetings.iter().collect();
    sorted.sort_by_key(|m: &&Meeting | m.start);

    sorted.windows(2).any(|pair| {
        let (a, b) = (pair[0], pair[1]);
        a.start < b.end && b.start < a.end
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::time;

    fn m(start: time::Time, end: time::Time) -> Meeting {
        Meeting { start, end }
    }

    #[test]
    fn no_meetings_no_conflict() {
        let meetings: Vec<Meeting> = vec![];
        assert!(!conflict_via_pairs(&meetings));
        assert!(!sort_conflict(&meetings));
    }

    #[test]
    fn single_meeting_no_conflict() {
        let meetings = vec![m(time!(9:00), time!(10:00))];
        assert!(!conflict_via_pairs(&meetings));
        assert!(!sort_conflict(&meetings));
    }

    #[test]
    fn two_separated_meetings_no_conflict() {
        let meetings = vec![m(time!(9:00), time!(10:00)), m(time!(11:00), time!(12:00))];
        assert!(!conflict_via_pairs(&meetings));
        assert!(!sort_conflict(&meetings));
    }

    #[test]
    fn back_to_back_meetings_are_not_a_conflict() {
        // one ends exactly when the other starts: 10:00-11:00 and 11:00-12:00
        let meetings = vec![m(time!(10:00), time!(11:00)), m(time!(11:00), time!(12:00))];
        assert!(!conflict_via_pairs(&meetings));
        assert!(!sort_conflict(&meetings));
    }

    #[test]
    fn back_to_back_meetings_out_of_order_are_not_a_conflict() {
        // same boundary case as above, but given out of chronological order,
        // to confirm sort_conflict actually sorts rather than trusting input order
        let meetings = vec![m(time!(11:00), time!(12:00)), m(time!(10:00), time!(11:00))];
        assert!(!conflict_via_pairs(&meetings));
        assert!(!sort_conflict(&meetings));
    }

    #[test]
    fn overlapping_meetings_conflict() {
        let meetings = vec![m(time!(10:00), time!(11:00)), m(time!(10:30), time!(11:30))];
        assert!(conflict_via_pairs(&meetings));
        assert!(sort_conflict(&meetings));
    }

    #[test]
    fn one_meeting_nested_inside_another_conflicts() {
        let meetings = vec![m(time!(9:00), time!(17:00)), m(time!(10:00), time!(10:30))];
        assert!(conflict_via_pairs(&meetings));
        assert!(sort_conflict(&meetings));
    }

    #[test]
    fn conflict_detected_even_when_not_adjacent_in_original_order() {
        // A is listed first but doesn't conflict with anything, B and C do
        // conflict, and only become adjacent after sorting by start time.
        let a = m(time!(13:00), time!(14:30));
        let b = m(time!(9:00), time!(10:30));
        let c = m(time!(10:00), time!(11:00));
        let meetings = vec![a, b, c];
        assert!(conflict_via_pairs(&meetings));
        assert!(sort_conflict(&meetings));
    }
}

