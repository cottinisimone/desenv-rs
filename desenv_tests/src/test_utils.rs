#[derive(Default)]
pub struct EnvUtil {
    envs: Vec<String>,
}

impl EnvUtil {
    #[must_use]
    pub fn new(env: &str, value: String) -> Self {
        std::env::set_var(env, value);
        Self { envs: vec![env.to_string()] }
    }
}

impl Drop for EnvUtil {
    fn drop(&mut self) {
        for env in &self.envs {
            std::env::remove_var(env);
        }
    }
}
