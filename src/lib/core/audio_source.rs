pub trait AudioSource {
    fn val_at_time(&self, t: f64) -> f64;
    fn duration(&self) -> f64;
}
