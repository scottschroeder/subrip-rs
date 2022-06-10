use std::time::Duration;

use crate::Subtitle;
pub fn out_of_order_subs(subs: &[Subtitle]) -> impl Iterator<Item = &Subtitle> {
    let mut t = Duration::new(0, 0);
    subs.iter().filter(move |r| {
        let bad_order = r.start < t || r.end < r.start;
        t = r.start;
        bad_order
    })
}

pub fn sort_subtitles(subs: &mut [Subtitle]) {
    subs.sort_by(|x, y| x.idx.cmp(&y.idx));
}

pub fn offset_subs(subs: &[Subtitle], delay_start: Option<Duration>) -> Vec<Subtitle> {
    if subs.is_empty() {
        return vec![];
    }
    let base = subs[0].start - delay_start.unwrap_or_default();
    subs.iter()
        .enumerate()
        .map(|(idx, s)| Subtitle {
            idx: idx as u32,
            start: s.start - base,
            end: s.end - base,
            text: s.text.clone(),
        })
        .collect()
}
