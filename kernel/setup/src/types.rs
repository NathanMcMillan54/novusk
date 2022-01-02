use konfig::Konfig;

pub type SetupReturn = (Result<(), &'static str>, &'static str);

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum SetupTypes {
    BmApp,
    Os,
    Server,
    Kernel,
    Nothing,
}

impl SetupTypes {
    fn server_init(self) {
        // TODO: Figure out how Novusk could be used to make a server
    }

    pub fn init(self) {
        let mut configs = Konfig::new();

        if self == SetupTypes::Os {
            sos::sos_init(configs.get("OS", "NAME").as_str());
        } else if self == SetupTypes::Server {
            self.server_init()
        } else if self == SetupTypes::BmApp {
            unsafe { libbmu::bmu_init(); }
        }
    }
}

pub(crate) fn str_to_setuptypes(setuptype: &str) -> SetupTypes {
    return match setuptype {
        "BmApp" => SetupTypes::BmApp,
        "Os" => SetupTypes::Os,
        "Server" => SetupTypes::Server,
        "Kernel" => SetupTypes::Kernel,
        "Nothing" => SetupTypes::Nothing,
        _ => SetupTypes::Nothing,
    };
}
