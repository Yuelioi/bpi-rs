/// Parameters for `/x/vip/web/vip_center/combine`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VipCenterInfoParams {
    build: u32,
}

impl VipCenterInfoParams {
    pub fn new() -> Self {
        Self { build: 0 }
    }

    pub fn with_build(mut self, build: u32) -> Self {
        self.build = build;
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("build", self.build.to_string())]
    }
}

impl Default for VipCenterInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vip_center_info_params_serializes_default_build() {
        let params = VipCenterInfoParams::new();

        assert_eq!(params.query_pairs(), vec![("build", "0".to_string())]);
    }

    #[test]
    fn vip_center_info_params_serializes_custom_build() {
        let params = VipCenterInfoParams::new().with_build(1);

        assert_eq!(params.query_pairs(), vec![("build", "1".to_string())]);
    }

    #[test]
    fn vip_center_info_params_allows_explicit_zero_build() {
        let params = VipCenterInfoParams::new().with_build(0);

        assert_eq!(params.query_pairs(), vec![("build", "0".to_string())]);
    }
}
