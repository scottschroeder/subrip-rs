/*!
  Helpers to perform common actions with subtitles
*/
use std::time::Duration;

use crate::Subtitle;

/// Find [`Subtitle`] records which appear out-of-order in the file
///
/// Most use-cases of this crate probably don't need to care, but it's
/// here if you need it.
pub fn out_of_order_subs(subs: &[Subtitle]) -> impl Iterator<Item = &Subtitle> {
    let mut t = Duration::new(0, 0);
    subs.iter().filter(move |r| {
        let bad_order = r.start < t || r.end < r.start;
        t = r.start;
        bad_order
    })
}

/// A helper to sort the subtitles by index
pub fn sort_subtitles(subs: &mut [Subtitle]) {
    subs.sort_by(|x, y| x.idx.cmp(&y.idx));
}

/// Shift the timestamps for the provided `subs` to 0 + `delay_start`.
///
/// This is useful if you are cutting a segment out of a video file, and need the relevant
/// subtitles to start at the beginning of the video.
///
/// `delay_start` is optional, and will insert a delay, otherwise the first subtitle card
/// will appear at 00:00:00.
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
