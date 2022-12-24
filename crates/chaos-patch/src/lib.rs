use {
    once_cell::sync::Lazy,
    serde::Deserialize,
    std::{fs, path::Path},
};

pub static PATCHES: Lazy<PatchManager> = Lazy::new(PatchManager::init);

#[derive(Debug)]
pub struct PatchManager {
    pub patches: Vec<Patch>,
}

impl PatchManager {
    fn init() -> Self {
        let mut patches = vec![];

        for file in fs::read_dir("patches").unwrap() {
            let path = file.unwrap().path();

            if path.extension().unwrap().to_string_lossy() == "toml" {
                patches.push(Patch::new(path));
            }
        }

        Self { patches }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Patch {
    pub data: Data,
    pub persistence: Persistence,
}

impl Patch {
    pub fn new(path: impl AsRef<Path>) -> Self {
        toml::from_slice(
            &fs::read(path)
                .unwrap_or_else(|path| panic!("failed to find file: {:?}", path.to_string())),
        )
        .unwrap()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Data {
    pub script: Option<String>,
    pub opcodes: Option<Vec<Opcode>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Opcode {
    pub address: (String, usize),
    pub on: Vec<u8>,
    pub off: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Persistence {
    pub chance: f64,
}
