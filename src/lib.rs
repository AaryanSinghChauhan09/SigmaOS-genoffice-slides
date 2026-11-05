#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transition { None, Fade, Slide, Zoom }

#[derive(Debug, Clone)]
pub struct Slide {
    pub index: usize,
    pub title: String,
    pub body: String,
    pub speaker_notes: String,
    pub transition: Transition,
}

impl Slide {
    pub fn new(index: usize, title: impl Into<String>) -> Self {
        Self { index, title: title.into(), body: String::new(), speaker_notes: String::new(), transition: Transition::None }
    }
    pub fn with_body(mut self, body: impl Into<String>) -> Self { self.body = body.into(); self }
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self { self.speaker_notes = notes.into(); self }
    pub fn with_transition(mut self, t: Transition) -> Self { self.transition = t; self }
}

#[derive(Debug, Default)]
pub struct Presentation {
    pub title: String,
    slides: Vec<Slide>,
}

impl Presentation {
    pub fn new(title: impl Into<String>) -> Self { Self { title: title.into(), slides: Vec::new() } }
    pub fn add_slide(&mut self, slide: Slide) { self.slides.push(slide); }
    pub fn slide_count(&self) -> usize { self.slides.len() }
    pub fn slide(&self, idx: usize) -> Option<&Slide> { self.slides.get(idx) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn presentation_add_slides() {
        let mut p = Presentation::new("SigmaOS Intro");
        p.add_slide(Slide::new(0, "Welcome").with_transition(Transition::Fade));
        assert_eq!(p.slide_count(), 1);
    }
}
