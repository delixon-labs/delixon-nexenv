use serde::Serialize;

/// Error estructurado que viaja al frontend con 4 campos uniformes.
/// Permite al usuario entender que intentaba Nexenv, que detecto, donde fallo y que hacer.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiError {
    pub intento: String,
    pub detecto: String,
    pub fallo: String,
    pub hacer: String,
}

impl UiError {
    pub fn new(intento: impl Into<String>) -> Self {
        Self {
            intento: intento.into(),
            detecto: String::new(),
            fallo: String::new(),
            hacer: String::new(),
        }
    }

    pub fn detecto(mut self, v: impl Into<String>) -> Self {
        self.detecto = v.into();
        self
    }

    pub fn fallo(mut self, v: impl Into<String>) -> Self {
        self.fallo = v.into();
        self
    }

    pub fn hacer(mut self, v: impl Into<String>) -> Self {
        self.hacer = v.into();
        self
    }
}

impl std::fmt::Display for UiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "intento={} detecto={} fallo={} hacer={}",
            self.intento, self.detecto, self.fallo, self.hacer
        )
    }
}

impl std::error::Error for UiError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_fills_all_fields() {
        let e = UiError::new("abrir proyecto")
            .detecto("falta carpeta")
            .fallo("ENOENT: no such file")
            .hacer("verificar que la ruta existe");
        assert_eq!(e.intento, "abrir proyecto");
        assert_eq!(e.detecto, "falta carpeta");
        assert_eq!(e.fallo, "ENOENT: no such file");
        assert_eq!(e.hacer, "verificar que la ruta existe");
    }

    #[test]
    fn serializes_with_camelcase_keys() {
        let e = UiError::new("a").detecto("b").fallo("c").hacer("d");
        let s = serde_json::to_string(&e).unwrap();
        assert!(s.contains("\"intento\""));
        assert!(s.contains("\"detecto\""));
        assert!(s.contains("\"fallo\""));
        assert!(s.contains("\"hacer\""));
    }

    #[test]
    fn empty_fields_default_to_empty_string() {
        let e = UiError::new("solo intento");
        assert_eq!(e.detecto, "");
        assert_eq!(e.fallo, "");
        assert_eq!(e.hacer, "");
    }

    #[test]
    fn implements_std_error() {
        let e = UiError::new("x");
        let _: &dyn std::error::Error = &e;
    }
}
