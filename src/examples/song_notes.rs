use fourier2::Note;

pub fn sample_song_notes() -> Vec<Note> {
  let mut notes = vec![];

  notes.extend(vec![
    Note::new("A.3".parse().unwrap(), 0.00, 1.00, 0.1),
    Note::new("C.3".parse().unwrap(), 0.25, 0.75, 0.1),
    Note::new("E.3".parse().unwrap(), 0.50, 0.50, 0.1),
    Note::new("A.4".parse().unwrap(), 0.75, 0.25, 0.1),
  ]);

  notes.extend(vec![
    Note::new("F.2".parse().unwrap(), 1.00, 1.00, 0.1),
    Note::new("A.3".parse().unwrap(), 1.25, 0.75, 0.1),
    Note::new("C.3".parse().unwrap(), 1.50, 0.50, 0.1),
    Note::new("F.3".parse().unwrap(), 1.75, 0.25, 0.1),
  ]);

  notes.extend(vec![
    Note::new("D.2".parse().unwrap(), 2.00, 1.00, 0.1),
    Note::new("F.2".parse().unwrap(), 2.25, 0.75, 0.1),
    Note::new("A.3".parse().unwrap(), 2.50, 0.50, 0.1),
    Note::new("D.3".parse().unwrap(), 2.75, 0.25, 0.1),
  ]);

  notes.extend(vec![
    Note::new("G.1".parse().unwrap(), 3.00, 2.00, 0.1),
    Note::new("B.2".parse().unwrap(), 3.25, 1.75, 0.1),
    Note::new("D.2".parse().unwrap(), 3.50, 1.50, 0.1),
    Note::new("G.2".parse().unwrap(), 3.75, 0.25, 0.1),
    Note::new("F.2".parse().unwrap(), 4.00, 1.00, 0.1),
  ]);

  notes
}
